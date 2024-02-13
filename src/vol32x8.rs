use wide::*;
use crate::consts::OptionDir;
use crate::bs32x8;
use crate::read_hist;
use bytemuck::cast;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
use crate::bs;

/* 
    Source: https://github.com/ronniec95/black_scholes

    Calculate call and put implied vol from an option price.
    Years to expiry should be expressed as a f32 such as 20 days is 20/252 = 0.79.
    Risk free rate, volatility and dividend yield expressed as f32 with 1.0 = 100%. 0.2 = 20% etc.

    Since there is no closed form solution, calculating the implied volatility is iterative and bounded by max_iterations. 
    The function exits when all the values in the array have reached a stable number.

    Threshold represents the largest acceptable difference between the calculated implied volatitilty 
    and actual implied Black volatility.
*/

#[wasm_bindgen]
pub fn implied_vol(
    option_dir: OptionDir,
    price: &[f32],
    spot: &[f32],
    strike: &[f32],
    risk_free_rate: &[f32],
    dividend_yield: &[f32],
    years_to_expiry: &[f32],
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
            1 < max_iterations &&
            0.0 < threshold
        )
    {
        return Vec::new();
    }

    let num_options = spot.len();

    if num_options == 0 {
        return Vec::new();
    }

    let implied_vol = (0..(num_options - 1) / 8 + 1)
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
                    max_iterations
                )
            );

            res[0..end_idx - start_idx].to_vec()
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

/* 
    Utilizes put call parity to calculate the average interest rate across a chain of options
    that are priced. Utilized for more accurate implied volatility calculations. 
*/
#[wasm_bindgen]
pub fn parity_interest_rate(
    call_price: &[f32],
    put_price: &[f32],
    spot: &[f32],
    strike: &[f32],
    years_to_expiry: &[f32]
) -> f32 {
    if
        !(
            call_price.len() == put_price.len() &&
            put_price.len() == spot.len() &&
            spot.len() == strike.len() &&
            strike.len() == years_to_expiry.len()
        )
    {
        return 0.0;
    }

    let num_options = spot.len();

    if num_options == 0 {
        return 0.0;
    }

    let rate = (0..(num_options - 1) / 8 + 1)
        .into_par_iter()
        .map(|idx| {
            let start_idx = idx * 8;
            let end_idx = std::cmp::min(num_options, start_idx + 8);

            if end_idx - start_idx < 8 {
                // Less than 8 elements left, so we calculate iteratively.
                // This is because if we call parity_interest_rate_f32x8 with less than 8 elements,
                // some of the elements will be set to NaN due to zero division.
                let mut sum = 0.0;

                for counter in 0..end_idx - start_idx {
                    sum += bs::parity_interest_rate(
                        call_price[start_idx + counter],
                        put_price[start_idx + counter],
                        spot[start_idx + counter],
                        strike[start_idx + counter],
                        years_to_expiry[start_idx + counter]
                    );
                }

                sum
            } else {
                // Utilize SIMD
                let call_price = f32x8::from(&call_price[start_idx..end_idx]);
                let put_price = f32x8::from(&put_price[start_idx..end_idx]);
                let spot = f32x8::from(&spot[start_idx..end_idx]);
                let strike = f32x8::from(&strike[start_idx..end_idx]);
                let years_to_expiry = f32x8::from(&years_to_expiry[start_idx..end_idx]);

                let res: f32x8 = bs32x8::parity_interest_rate_f32x8(
                    call_price,
                    put_price,
                    spot,
                    strike,
                    years_to_expiry
                );

                res.reduce_add()
            }
        })
        .reduce(
            || 0.0,
            |mut acc: f32, x: f32| {
                // Parallel reduction, in this case, appending results to the accumulator
                acc + x
            }
        );

    rate / (num_options as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bs;
    use bytemuck::cast;

    #[test]
    fn interest_rate_check_small() {
        let rate = parity_interest_rate(
            &[8.247, 8.247],
            &[5.785, 5.785],
            &[100.0, 100.0],
            &[100.0, 100.0],
            &[0.5, 0.5]
        );

        let expected_rate = 0.0498;

        assert!(
            (rate - expected_rate).abs() < 0.0001,
            "Got: {}, Expected: {}",
            rate,
            expected_rate
        );
    }

    #[test]
    fn interest_rate_check_medium() {
        let rate = parity_interest_rate(
            &[8.247, 8.247, 8.247, 8.247, 8.247, 8.247, 8.247, 8.247],
            &[5.785, 5.785, 5.785, 5.785, 5.785, 5.785, 5.785, 5.785],
            &[100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0],
            &[100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0],
            &[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5]
        );

        let expected_rate = 0.0498;

        assert!(
            (rate - expected_rate).abs() < 0.0001,
            "Got: {}, Expected: {}",
            rate,
            expected_rate
        );
    }

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
            20,
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
            20,
            0.0001
        );

        assert!(vol.len() == 8, "Num results: {}", vol.len());
    }

    #[test]
    fn implied_vol_check_large() {
        let (spot, call_prices, call_strikes, put_prices, put_strikes, years_to_expiry, _) =
            read_hist::get_appl_data();

        let n = call_prices.len();
        let spot: Vec<f32> = vec![spot; n];
        let risk_free_rate: Vec<f32> = vec![0.01; n];
        let dividend_yield: Vec<f32> = vec![0.0; n];

        let vol = implied_vol(
            OptionDir::CALL,
            &call_prices,
            &spot,
            &call_strikes,
            &risk_free_rate,
            &dividend_yield,
            &years_to_expiry,
            20,
            0.0001
        );

        assert!(vol.len() == n, "Num results: {}", n);
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
            &[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
            20,
            0.0001
        );

        assert!(vol.len() == 0, "Num results: {}", vol.len());
    }
}
