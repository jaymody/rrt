A ray tracer built in Rust that can run in your browser via WebAssembly.

Based on:
- [My rust implementation of Ray Tracing In One Weekend](https://github.com/jaymody/ray-tracing-rust)
- [My experiments with Rust + WebAssembly](https://github.com/jaymody/game-of-life)
- [The excellent rpt ray tracer by Eric Zhang and Alexander Morozov](https://github.com/ekzhang/rpt).

### Usage
Run the ray tracer natively in rust:
```shell
cargo run --example main
```

### Profiling
We can profile the code using [flamegraph](https://github.com/flamegraph-rs/flamegraph) (Note: [On MacOS I had to add the `--root` flag to make flamegraph work](https://github.com/flamegraph-rs/flamegraph/issues/31#issuecomment-589256677)):
```shell
cargo flamegraph --release --example main
```
