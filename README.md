A ray tracer built in Rust that can run in your browser via WebAssembly.

Based on:
- [My rust implementation of Ray Tracing In One Weekend](https://github.com/jaymody/ray-tracing-rust)
- [My experiments with Rust + WebAssembly](https://github.com/jaymody/game-of-life)
- [The excellent rpt ray tracer by Eric Zhang and Alexander Morozov](https://github.com/ekzhang/rpt).

### Usage
This project requires:

- `rust`: Language of choice to compile to wasm ([installation instructions](https://doc.rust-lang.org/book/ch01-01-installation.html)).
- `wasm-pack`: Compiles rust code a wasm package we can use in javascript ([installation instructions](https://rustwasm.github.io/wasm-pack/installer/)).
- `vite`: To bundle and serve the webpage (`npm install -g vite`).

Run the ray tracer natively in rust:
```shell
cargo run --example --release scene1
```

We can profile the code using [flamegraph](https://github.com/flamegraph-rs/flamegraph) (Note: [On MacOS I had to add the `--root` flag to make flamegraph work](https://github.com/flamegraph-rs/flamegraph/issues/31#issuecomment-589256677)):
```shell
cargo flamegraph --release --example scene1
```

To build the wasm js package:
```shell
wasm-pack build --target web --release crates/rrt-wasm
```

And finally, to actually run the webpage:
```shell
vite
```
