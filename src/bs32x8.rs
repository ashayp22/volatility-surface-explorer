use wide::*;

/// Specify whether an option is put or call
#[derive(PartialEq, Debug, Copy, Clone, PartialOrd)]
pub enum OptionDir {
    CALL = 1,
    PUT = -1,
}

// Source: https://github.com/ronniec95/black_scholes

fn erf_f32x8(x: f32x8) -> f32x8 {
    let t = x.sign_bit();
    let e = x.abs();
    let n: f32x8 = f32x8::splat(0.3275911);
    let a: f32x8 = f32x8::splat(0.254829592);
    let r: f32x8 = f32x8::splat(-0.284496736);
    let i: f32x8 = f32x8::splat(1.421413741);
    let l: f32x8 = f32x8::splat(-1.453152027);
    let d: f32x8 = f32x8::splat(1.061405429);
    let u = f32x8::ONE / e.mul_add(n, f32x8::ONE);
    let eu = u * (-e * e).exp();
    let m = eu.mul_neg_add(u.mul_add(u.mul_add(u.mul_add(d.mul_add(u, l), i), r), a), f32x8::ONE);
    t.blend(-m, m)
}

fn phi_f32x8(e: f32x8) -> f32x8 {
    let v = f32x8::HALF * (f32x8::ONE + erf_f32x8(e / f32x8::SQRT_2));
    let min: f32x8 = f32x8::splat(-1.0e5);
    let max: f32x8 = f32x8::splat(1.0e5);

    let zero_mask = e.cmp_lt(min);
    let one_mask = e.cmp_gt(max);
    let v = zero_mask.blend(f32x8::ZERO, v);
    let v = one_mask.blend(f32x8::ONE, v);
    v
}

fn pdf_f32x8(x: f32x8, mu: f32x8, sigma: f32x8) -> f32x8 {
    const P: f32 = 2.506628274631000502415765284811;
    ((-1.0 * (x - mu) * (x - mu)) / (2.0 * sigma * sigma)).exp() / (sigma * P)
}

fn call_price_f32x8(
    spot: f32x8,
    strike: f32x8,
    volatility: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
    years_to_expiry: f32x8
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = (1.0 / rd) * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;
    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;

    // Call specific
    let o = phi_f32x8(d1);
    let c = phi_f32x8(d2);
    o * spot * la - c * g
}

fn put_price_f32x8(
    spot: f32x8,
    strike: f32x8,
    volatility: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
    years_to_expiry: f32x8
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = (f32x8::ONE / rd) * (ssln + (il + vs2) * years_to_expiry);
    let d2 = d1 - rd;

    let la = (-dividend_yield * years_to_expiry).exp();
    let ia = (-risk_free_rate * years_to_expiry).exp();
    let g = strike * ia;
    // Put specific
    let o = phi_f32x8(-d1);
    let c = phi_f32x8(-d2);
    c * g - o * spot * la
}

fn vega_f32x8(
    spot: f32x8,
    strike: f32x8,
    volatility: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
    years_to_expiry: f32x8
) -> f32x8 {
    let d = years_to_expiry.sqrt();
    let rd = volatility * d;
    let vs2 = (volatility * volatility) / 2.0;
    let ssln = (spot / strike).ln();
    let il = risk_free_rate - dividend_yield;
    let d1 = (f32x8::ONE / rd) * (ssln + (il + vs2) * years_to_expiry);
    let v = pdf_f32x8(d1, f32x8::ZERO, f32x8::ONE);
    let la = (-dividend_yield * years_to_expiry).exp();
    spot * la * v * d
}

pub(crate) fn price_f32x8(
    dir: OptionDir,
    spot: f32x8,
    strike: f32x8,
    volatility: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
    years_to_expiry: f32x8
) -> f32x8 {
    match dir {
        OptionDir::CALL =>
            call_price_f32x8(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            ),
        OptionDir::PUT =>
            put_price_f32x8(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            ),
    }
}

pub(crate) fn implied_vol_f32x8(
    option_dir: OptionDir,
    price: f32x8,
    spot: f32x8,
    strike: f32x8,
    risk_free_rate: f32x8,
    dividend_yield: f32x8,
    years_to_expiry: f32x8,
    diff_threshold: f32,
    max_iterations: i32,
    initial_volatility: f32
) -> f32x8 {
    let mut volatility = f32x8::splat(initial_volatility);
    let mut count = 0;

    // Apply the Newton-Raphon Method
    loop {
        let option_value = price_f32x8(
            option_dir,
            spot,
            strike,
            volatility,
            risk_free_rate,
            dividend_yield,
            years_to_expiry
        );
        let diff = option_value - price;
        let mask = diff.abs().cmp_lt(f32x8::splat(diff_threshold));
        if mask.all() {
            break;
        }
        let derivative = vega_f32x8(
            spot,
            strike,
            volatility,
            risk_free_rate,
            dividend_yield,
            years_to_expiry
        );
        // let derivative = derivative.max(f32x8::ONE);
        let bump_value = diff / derivative;
        volatility = volatility - bump_value;
        if count > max_iterations {
            break;
        } else {
            count = count + 1;
        }
    }
    let vol_mask = volatility.cmp_gt(f32x8::ZERO);
    volatility = vol_mask.blend(volatility, f32x8::ZERO);
    let price_mask = price.cmp_eq(f32x8::ZERO);
    volatility = price_mask.blend(f32x8::ZERO, volatility);
    volatility
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bs;
    use bytemuck::cast;

    #[test]
    fn put_price_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = bs::put_price(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            );

            let actual: [f32; 8] = cast(
                price_f32x8(
                    OptionDir::PUT,
                    spot.into(),
                    strike.into(),
                    volatility.into(),
                    risk_free_rate.into(),
                    dividend_yield.into(),
                    years_to_expiry.into()
                )
            );
            println!("Put {} price {:?}", actual[0], expected);
            assert!((actual[0] - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn call_price_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = bs::call_price(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            );

            let actual: [f32; 8] = cast(
                price_f32x8(
                    OptionDir::CALL,
                    spot.into(),
                    strike.into(),
                    volatility.into(),
                    risk_free_rate.into(),
                    dividend_yield.into(),
                    years_to_expiry.into()
                )
            );
            println!("Call {} price {:?}", actual[0], expected);
            assert!((actual[0] - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn vega_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.01;

            // Basic call/put test
            let expected = bs::vega(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            );

            let actual: [f32; 8] = cast(
                vega_f32x8(
                    spot.into(),
                    strike.into(),
                    volatility.into(),
                    risk_free_rate.into(),
                    dividend_yield.into(),
                    years_to_expiry.into()
                )
            );
            assert!((actual[0] - expected).abs() < 100.0);
        }
    }

    #[test]
    fn check_put_iv_from_price() {
        let spot = 131.0;
        let strike = 115.0;
        let years_to_expiry = 24.0 / 252.0;
        let risk_free_rate = 0.001;
        let volatility = 0.419;
        let dividend_yield = 0.00625 * 12.0;

        // Basic call/put test
        let put_price = bs::put_price(
            spot,
            strike,
            volatility,
            risk_free_rate,
            dividend_yield,
            years_to_expiry
        );
        let v: [f32; 8] = cast(
            implied_vol_f32x8(
                OptionDir::PUT,
                put_price.into(),
                spot.into(),
                strike.into(),
                risk_free_rate.into(),
                dividend_yield.into(),
                years_to_expiry.into(),
                0.00001,
                5,
                0.2
            )
        );
        println!("Put {} IV {:?}", put_price, v[0]);
        assert!((v[0] - volatility).abs() < 0.00001);
    }

    #[test]
    fn check_call_iv_from_price() {
        let spot = 131.0;
        let strike = 115.0;
        let years_to_expiry = 24.0 / 252.0;
        let risk_free_rate = 0.001;
        let volatility = 0.419;
        let dividend_yield = 0.00625 * 12.0;

        // Basic call/put test
        let call_price = bs::call_price(
            spot,
            strike,
            volatility,
            risk_free_rate,
            dividend_yield,
            years_to_expiry
        );
        let v: [f32; 8] = cast(
            implied_vol_f32x8(
                OptionDir::CALL,
                call_price.into(),
                spot.into(),
                strike.into(),
                risk_free_rate.into(),
                dividend_yield.into(),
                years_to_expiry.into(),
                0.00001,
                5,
                0.2
            )
        );
        println!("Put {} IV {:?}", call_price, v[0]);
        assert!((v[0] - volatility).abs() < 0.00001);
    }
}
