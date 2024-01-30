use wide::*;
use crate::consts::OptionDir;
use crate::bs32x8;
use crate::read_hist;
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
    // Check parameters
    if
        !(
            price.len() == spot.len() &&
            spot.len() == strike.len() &&
            strike.len() == risk_free_rate.len() &&
            risk_free_rate.len() == dividend_yield.len() &&
            dividend_yield.len() == years_to_expiry.len() &&
            years_to_expiry.len() == prev_implied_vol.len() &&
            1 < max_iterations &&
            0.0 < threshold
        )
    {
        return Vec::new();
    }

    let num_options = spot.len();

    let implied_vol = (0..num_options / 8 + 1)
        .into_par_iter()
        .map(|idx| {
            let start_idx = idx * 8;
            let end_idx = std::cmp::min(num_options, start_idx + 8);

            let price = f32x8::from(&price[start_idx..end_idx]);
            let spot = f32x8::from(&spot[start_idx..end_idx]);
            let strike = f32x8::from(&strike[start_idx..end_idx]);
            let years_to_expiry = f32x8::from(&years_to_expiry[start_idx..end_idx]);
            let risk_free_rate = f32x8::from(&risk_free_rate[start_idx..end_idx]);
            let dividend_yield = f32x8::from(&dividend_yield[start_idx..end_idx]);
            let prev_implied_vol = f32x8::from(&prev_implied_vol[start_idx..end_idx]);

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

            res[0..end_idx-start_idx].to_vec()
        })
        .reduce(
            || Vec::new(),
            |mut acc: Vec<f32>, x: Vec<f32>| {
                // Parallel reduction, in this case, appending results to the accumulator
                acc.extend_from_slice(&x);
                acc
            }
        );

    implied_vol
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bs;
    use bytemuck::cast;

    #[test]
    fn implied_vol_check_small() {
        let vol = implied_vol(
            OptionDir::CALL,
            &[4.0],
            &[125.0],
            &[120.0],
            &[0.02],
            &[0.0],
            &[0.5],
            &[0.2],
            5,
            0.0001
        );

        assert!(vol.len() == 1, "Num results: {}", vol.len());
    }

    #[test]
    fn implied_vol_check_medium() {
        let vol = implied_vol(
            OptionDir::CALL,
            &[4.0, 3.0, 2.0, 3.0, 4.0, 3.0, 2.0, 3.0],
            &[125.0, 120.0, 125.0, 120.0, 125.0, 120.0, 125.0, 120.0],
            &[120.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0],
            &[0.02, 0.025, 0.03, 0.035, 0.02, 0.025, 0.03, 0.035],
            &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            &[0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2],
            5,
            0.0001
        );

        assert!(vol.len() == 8, "Num results: {}", vol.len());
    }

    #[test]
    fn implied_vol_check_large() {
        let (spot, call_prices, call_strikes, put_prices, put_strikes, years_to_expiry) =
            read_hist::get_appl_data();

        let n = call_prices.len();
        let spot: Vec<f32> = vec![spot; n];
        let risk_free_rate: Vec<f32> = vec![0.01; n];
        let dividend_yield: Vec<f32> = vec![0.0; n];
        let prev_implied_vol: Vec<f32> = vec![0.2; n];

        let vol = implied_vol(
            OptionDir::CALL,
            &call_prices,
            &spot,
            &call_strikes,
            &risk_free_rate,
            &dividend_yield,
            &years_to_expiry,
            &prev_implied_vol,
            5,
            0.0001
        );

        // assert!(vol.len() == n, "Num results: {}", n);
    }

    #[test]
    fn implied_vol_check_bad() {
        let vol = implied_vol(
            OptionDir::CALL,
            &[4.0, 3.0, 2.0, 3.0, 4.0, 3.0, 2.0, 3.0],
            &[125.0, 120.0, 125.0, 120.0, 125.0, 120.0, 125.0, 120.0],
            &[120.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0, 120.0],
            &[0.02, 0.025, 0.03, 0.035, 0.02, 0.025, 0.03, 0.035],
            &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            &[0.2, 0.2],
            5,
            0.0001
        );

        assert!(vol.len() == 0, "Num results: {}", vol.len());
    }
}
