# Implied Volatility Surface Explorer with SIMD and multithreading

![Implied Volatility Surface Explorer website](demo/home.png)

A fast, practical Implied Volatility Surface Explorer for SPY and similar options from the [CBOE quotes dashboard](https://www.cboe.com/delayed_quotes/spy/quote_table). Implied Volatility is calculated using [SIMD](https://docs.rs/wide/latest/wide/) and [Rayon](https://github.com/rayon-rs/rayon) and ported to the web with [WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm). Surfaces are plotted using [Plotly](https://plotly.com/javascript/).

# Usage

### Local

To run locally, first compile from Rust to WASM with (if your machine doesn't support WASM, remove the RUSTFLAGS):

```sh
RUSTFLAGS='-C target-feature=+simd128' wasm-pack build --target web
```

This will generate a `pkg` directory. Then, run the website locally:

```sh
npx serve
```

You may view the sample data or upload the most recent SPY data from CBOE. To do this, visit the [CBOE delayed quotes website](https://www.cboe.com/delayed_quotes/spy/quote_table):

![Implied Volatility Surface Explorer website](demo/step1.png)

Then, update the Volume, Expiration Type, Options Range, Size, and Expiration filters as desired and scroll to the bottom to Download CSV.

![Implied Volatility Surface Explorer website](demo/step2.png)

Finally, download the CSV file and upload it to the website.

### Performance and Testing

To run the Rust benchmark locally, comment out these lines in the Cargo.toml

```toml
[lib]
crate-type = ["cdylib"]
```

and then run

```sh
cargo bench
```

Similarily, for tests:

```sh
cargo test
```

# Performance

Calculated on a Macbook M1:

For 112 spots, 80 trials and 100 steps (~11x improvement in speed):

- `mc::call_price()`: ~33.649ms
- `mc_simd::call_price()`: ~3.29ms

For 112 spots, 1000 trials and 100 steps (~27x improvement in speed):

- `mc::call_price()`: ~419ms
- `mc_simd::call_price()`: ~15ms

For 112 spots, 10000 trials and 100 steps (~40x improvement in speed):

- `mc::call_price()`: ~4.26s
- `mc_simd::call_price()`: ~104ms

For 112 spots, 2000 trials and 1000 steps (~40x improvement in speed):

- `mc::call_price()`: ~8.22s
- `mc_simd::call_price()`: ~205ms

For 112 spots, 20000 trials and 100 steps (~40x improvement in speed):

- `mc::call_price()`: ~8.27s
- `mc_simd::call_price()`: ~207ms

Notice that `mc_simd` performance increases compared to `mc` as the number of trials and steps get larger.

# Extensions

1. In real-life applications, the implied volatility calculation should happen in the backend in order to full utilize SIMD instructions and multithreading. Instead of being converted to WASM, the Rust code can be deployed in a backend system and connected to the frontend with HTTP or another protocol.
2. If the Rust web port is kept, multithreading can be utilized in the frontend by enabling `SharedArrayBuffer` in the browser, adding [wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon), and re-compiling to WASM.

# License

[GNU GPL v3](LICENSE)