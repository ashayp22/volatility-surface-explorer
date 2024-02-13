use crate::consts::OptionDir;

// Black-scholes pricer, used to test the other function
fn erf(x: f32) -> f32 {
    let t = x.signum();
    let e = x.abs();
    const N: f32 = 0.3275911;
    const A: f32 = 0.254829592;
    const R: f32 = -0.284496736;
    const I: f32 = 1.421413741;
    const L: f32 = -1.453152027;
    const D: f32 = 1.061405429;
    let u = 1.0 / (1.0 + N * e);
    let m = 1.0 - ((((D * u + L) * u + I) * u + R) * u + A) * u * (-e * e).exp();
    t * m
}

fn normal_cdf(x: f32) -> f32 {
    0.5 * (1.0 + erf(x / (2.0f32).sqrt()))
}

fn pdf(x: f32, mu: f32, sigma: f32) -> f32 {
    ((-1.0 * (x - mu) * (x - mu)) / (2.0 * sigma * sigma)).exp() /
        (sigma * (2.0 * 3.14159f32).sqrt())
}

fn d(
    spot: f32,
    strike: f32,
    volatility: f32,
    risk_free_rate: f32,
    dividend_yield: f32,
    years_to_expiry: f32
) -> (f32, f32) {
    let d1: f32 =
        ((spot / strike).ln() +
            (risk_free_rate - dividend_yield + (volatility * volatility) / 2.0) * years_to_expiry) /
        (volatility * years_to_expiry.sqrt());
    let d2: f32 = d1 - volatility * years_to_expiry.sqrt();
    (d1, d2)
}

pub(crate) fn call_price(
    spot: f32,
    strike: f32,
    volatility: f32,
    risk_free_rate: f32,
    dividend_yield: f32,
    years_to_expiry: f32
) -> f32 {
    let (d1, d2) = d(spot, strike, volatility, risk_free_rate, dividend_yield, years_to_expiry);

    let call: f32 =
        spot * (-dividend_yield * years_to_expiry).exp() * normal_cdf(d1) -
        strike * (-risk_free_rate * years_to_expiry).exp() * normal_cdf(d2);
    call
}

pub(crate) fn put_price(
    spot: f32,
    strike: f32,
    volatility: f32,
    risk_free_rate: f32,
    dividend_yield: f32,
    years_to_expiry: f32
) -> f32 {
    let (d1, d2) = d(spot, strike, volatility, risk_free_rate, dividend_yield, years_to_expiry);

    let put: f32 =
        strike * (-risk_free_rate * years_to_expiry).exp() * (1.0 - normal_cdf(d2)) -
        spot * (-dividend_yield * years_to_expiry).exp() * (1.0 - normal_cdf(d1));
    put
}

pub(crate) fn price(
    dir: OptionDir,
    spot: f32,
    strike: f32,
    volatility: f32,
    risk_free_rate: f32,
    dividend_yield: f32,
    years_to_expiry: f32
) -> f32 {
    match dir {
        OptionDir::CALL =>
            call_price(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            ),
        OptionDir::PUT =>
            put_price(
                spot,
                strike,
                volatility,
                risk_free_rate,
                dividend_yield,
                years_to_expiry
            ),
    }
}

pub(crate) fn vega(
    spot: f32,
    strike: f32,
    volatility: f32,
    risk_free_rate: f32,
    dividend_yield: f32,
    years_to_expiry: f32
) -> f32 {
    let (d1, _) = d(spot, strike, volatility, risk_free_rate, dividend_yield, years_to_expiry);
    let nd1 = pdf(d1, 0.0, 1.0);
    (-dividend_yield * years_to_expiry).exp() * nd1 * (spot * years_to_expiry.sqrt())
}


pub fn implied_vol(
    option_dir: OptionDir,
    option_price: &[f32],
    spot: &[f32],
    strike: &[f32],
    risk_free_rate: &[f32],
    dividend_yield: &[f32],
    years_to_expiry: &[f32],
    max_iterations: i32,
    threshold: f32
) -> Vec<f32> {
    let n = option_price.len();
    let mut impl_vol = Vec::new();

    for i in 0..n {
        let mut count = 0;
        let mut low = 0.0;
        let mut high = 5.0;

        loop {
            let mid = (low + high) / 2.0;
            let option_value = price(option_dir, spot[i], strike[i], mid, risk_free_rate[i], dividend_yield[i], years_to_expiry[i]);

            if option_value > option_price[i] {
                high = mid;
            } else {
                low = mid;
            }

            if count > max_iterations {
                break;
            } else {
                count = count + 1;
            }
        }

        impl_vol.push((low + high) / 2.0);
    }

    impl_vol
}