use crate::{
    vol32x8,
    consts
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("implied volatility f32x8", |b| b.iter(|| {
        let price = vec![3.15; 25000];
        let spot = vec![545.0; 25000];
        let strike = vec![550.0; 25000];
        let risk_free_rate = vec![0.01; 25000];
        let dividend_yield = vec![0.0; 25000];
        let years_to_expiry = vec![0.5; 25000];
        let prev_implied_vol = vec![0.2; 25000];

        let _ = implied_vol(
            option_dir: consts::OptionDir::CALL,
            price: price,
            spot: spot,
            strike: strike,
            risk_free_rate: risk_free_rate,
            dividend_yield: dividend_yield,
            years_to_expiry: years_to_expiry,
            prev_implied_vol: prev_implied_vol,
            max_iterations: 20,
            threshold: 0.001
        );
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);