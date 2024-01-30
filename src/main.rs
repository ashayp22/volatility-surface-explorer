use simd_vol::read_hist;
use simd_vol::consts::OptionDir;
use simd_vol::vol32x8;

fn main() {
    let (spot, call_prices, call_strikes, put_prices, put_strikes, years_to_expiry) =
        read_hist::get_appl_data();

    let n = call_prices.len();
    let spot: Vec<f32> = vec![spot; n];
    let risk_free_rate: Vec<f32> = vec![0.01; n];
    let dividend_yield: Vec<f32> = vec![0.0; n];
    let prev_implied_vol: Vec<f32> = vec![0.2; n];

    let vol = vol32x8::implied_vol(
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

    println!("{:?}", vol);
}
