use wide::*;
use crate::bs32x8::OptionDir;
use crate::bs32x8;
use bytemuck::cast;
use rayon::prelude::*;

/* 
    Source: https://github.com/ronniec95/black_scholes

    Calculate call and put implied vol from an option price.
    Years to expiry should be expressed as a f32 such as 20 days is 20/252 = 0.79.
    Risk free rate, volatility and dividend yield expressed as f32 with 1.0 = 100%. 0.2 = 20% etc.

    Since there is no closed form solution, calculating the implied volatility is iterative and bounded by max_iterations. 
    The function exits when all the values in the array have reached a stable number.

    Threshold represents the largest acceptable difference between the calculated implied volatitilty 
    and actual implied Black volatility.

    prev_implied_vol represents the previous implied volatilities, and if chosen correctly, can limit the iteration
    to 1 per price/spot/strike combination. Generally, the algorithm converges between 2-3 iterations.
*/

pub fn implied_vol(
    option_dir: OptionDir,
    price: &[f32],
    spot: &[f32],
    strike: &[f32],
    risk_free_rate: &[f32],
    dividend_yield: &[f32],
    years_to_expiry: &[f32],
    prev_implied_vol: &[f32],
    max_iterations: i32,
    threshold: f32
) -> Vec<f32> {
    let max_idx = spot.len();

    let irvol = (0..max_idx / 8)
        .into_par_iter()
        .map(|idx| {
            let i = idx * 8;

            let price = f32x8::from(&price[i..std::cmp::min(max_idx, i + 8)]);
            let spot = f32x8::from(&spot[i..std::cmp::min(max_idx, i + 8)]);
            let strike = f32x8::from(&strike[i..std::cmp::min(max_idx, i + 8)]);
            let years_to_expiry = f32x8::from(&years_to_expiry[i..std::cmp::min(max_idx, i + 8)]);
            let risk_free_rate = f32x8::from(&risk_free_rate[i..std::cmp::min(max_idx, i + 8)]);
            let dividend_yield = f32x8::from(&dividend_yield[i..std::cmp::min(max_idx, i + 8)]);
            let prev_implied_vol = f32x8::from(&prev_implied_vol[i..std::cmp::min(max_idx, i + 8)]);

            let res: [f32; 8] = cast(
                bs32x8::implied_vol_f32x8(
                    option_dir,
                    price,
                    spot,
                    strike,
                    risk_free_rate,
                    dividend_yield,
                    years_to_expiry,
                    threshold,
                    max_iterations,
                    prev_implied_vol
                )
            );

            res.to_vec()
        })
        .reduce(
            || Vec::new(),
            |mut acc: Vec<f32>, x: Vec<f32>| {
                // Parallel reduction, in this case, appending results to the accumulator
                acc.extend_from_slice(&x);
                acc
            }
        );

    irvol
}
