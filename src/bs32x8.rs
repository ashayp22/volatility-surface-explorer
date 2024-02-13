use wide::*;
use crate::consts::OptionDir;

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
    max_iterations: i32
) -> f32x8 {
    let mut count = 0;

    // Min volatility of 0%, max volatility of 500%
    let mut low = f32x8::splat(0.0);
    let mut high = f32x8::splat(5.0);
    let two_f32x8 = f32x8::splat(2.0);
    let one_f32x8 = f32x8::splat(1.0);

    // Run bisection method
    loop {
        let mask = (high - low).abs().cmp_lt(f32x8::splat(diff_threshold));
        if mask.all() {
            break;
        }

        let middle = (high + low) / two_f32x8;

        let option_value = price_f32x8(
            option_dir,
            spot,
            strike,
            middle,
            risk_free_rate,
            dividend_yield,
            years_to_expiry
        );

        // 0 if diff is positive, 1 if diff is negative
        // 0 means we need to update high, 1 means we need to update low
        let is_positive_diff = (option_value - price)
            .cmp_lt(f32x8::splat(0.0))
            .is_nan()
            .min(f32x8::splat(1.0));
        let is_negative_diff = one_f32x8 - is_positive_diff;

        low = is_positive_diff * middle + is_negative_diff * low;
        high = is_negative_diff * middle + is_positive_diff * high;

        if count > max_iterations {
            break;
        } else {
            count = count + 1;
        }
    }

    (high + low) / two_f32x8
}

// Apply put call parity to determine interest rate
pub(crate) fn interest_rate_f32x8(
    call_price: f32x8,
    put_price: f32x8,
    spot: f32x8,
    strike: f32x8,
    years_to_expiry: f32x8
) -> f32x8 {
    return (strike / (spot - call_price + put_price)).ln() / years_to_expiry;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bs;
    use bytemuck::cast;

    #[test]
    fn interest_rate_check_small() {
        let interest_rate: [f32; 8] = cast(
            interest_rate_f32x8(
                (8.247).into(),
                (5.785).into(),
                (100.0).into(),
                (100.0).into(),
                (0.5).into()
            )
        );

        let expected_rate = 0.04985638;

        assert!(
            (interest_rate[0] - expected_rate).abs() < 0.01,
            "Got: {}, Expected: {}",
            interest_rate[0],
            expected_rate
        );
    }

    #[test]
    fn interest_rate_check() {
        for i in (50..90).step_by(1) {
            let spot = 50.0;
            let strike = i as f32;
            let years_to_expiry = 1.0;
            let risk_free_rate = 0.02;
            let volatility = 0.2;
            let dividend_yield = 0.0;

            let put_price = price_f32x8(
                OptionDir::PUT,
                spot.into(),
                strike.into(),
                volatility.into(),
                risk_free_rate.into(),
                dividend_yield.into(),
                years_to_expiry.into()
            );

            let call_price = price_f32x8(
                OptionDir::CALL,
                spot.into(),
                strike.into(),
                volatility.into(),
                risk_free_rate.into(),
                dividend_yield.into(),
                years_to_expiry.into()
            );

            let interest_rate: [f32; 8] = cast(
                interest_rate_f32x8(
                    call_price,
                    put_price,
                    spot.into(),
                    strike.into(),
                    years_to_expiry.into()
                )
            );

            assert!((interest_rate[0] - risk_free_rate).abs() < 0.0001);
        }
    }

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
            assert!((actual[0] - expected).abs() < 1.0);
        }
    }

    #[test]
    fn check_put_iv_from_price() {
        let spot = 131.0;
        let strike = 115.0;
        let years_to_expiry = 2.5;
        let risk_free_rate = 0.01;
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
                20
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
        let risk_free_rate = 0.01;
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
                100
            )
        );
        println!("Call {} IV {:?}", call_price, v[0]);
        assert!((v[0] - volatility).abs() < 0.00001);
    }

    #[test]
    fn check_call_iv_from_price_2() {
        let spot = 546.0255;
        let strike = 490.0;
        let years_to_expiry = 0.030136986;
        let risk_free_rate = 0.01;
        let dividend_yield = 0.0;

        let v: [f32; 8] = cast(
            implied_vol_f32x8(
                OptionDir::CALL,
                (55.45).into(),
                spot.into(),
                strike.into(),
                risk_free_rate.into(),
                dividend_yield.into(),
                years_to_expiry.into(),
                0.00001,
                100
            )
        );
        assert!((v[0] - 0.0).abs() < 0.00001);
    }
}
