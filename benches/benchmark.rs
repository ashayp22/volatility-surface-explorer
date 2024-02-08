use simd_vol::{
    vol32x8,
    consts,
    bs,
    read_hist
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let (spot, call_prices, call_strikes, _, _, years_to_expiry, _) =
    read_hist::get_appl_data();

    let n = call_prices.len();
    let spot: Vec<f32> = vec![spot; n];
    let risk_free_rate: Vec<f32> = vec![0.01; n];
    let dividend_yield: Vec<f32> = vec![0.0; n];

    c.bench_function("implied volatility single", |b| b.iter(|| {
        let _ = bs::implied_vol(
            consts::OptionDir::CALL,
            &call_prices,
            &spot,
            &call_strikes,
            &risk_free_rate,
            &dividend_yield,
            &years_to_expiry,
            20,
            0.001
        );
    }));

    c.bench_function("implied volatility f32x8", |b| b.iter(|| {
        let _ = vol32x8::implied_vol(
            consts::OptionDir::CALL,
            &call_prices,
            &spot,
            &call_strikes,
            &risk_free_rate,
            &dividend_yield,
            &years_to_expiry,
            20,
            0.001
        );
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);