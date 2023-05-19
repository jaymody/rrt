A ray tracer built in Rust that can run in your browser via WebAssembly.

Based on:
- [My rust implementation of Ray Tracing In One Weekend](https://github.com/jaymody/ray-tracing-rust)
- [My experiments with Rust + WebAssembly](https://github.com/jaymody/game-of-life)
- [The excellent rpt ray tracer by Eric Zhang and Alexander Morozov](https://github.com/ekzhang/rpt).
- [My experiments with `wasm-bindgen-rayon` to support multi-threading in Rust compiled wasm via rayon](https://github.com/jaymody/rust-rayon-wasm-demo/tree/main).

### Usage
Run the ray tracer natively in rust:
```shell
cargo run --example --release scene1
```

To run the frontend with `rrt` compiled as wasm:
```shell
npm install
npm run build-wasm
npm run serve
```
