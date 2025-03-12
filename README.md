This is a minimal reproduction of a bug in wasm-bindgen.

## Requirements:

- rustc 1.83.0
- cargo 1.83.0
- wasm-bindgen-cli 0.2.100

To reproduce:

```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/foo.wasm --out-dir "target/pkg"
```

```bash
thread 'main' panicked at /Users/REDACTED/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasm-bindgen-cli-support-0.2.100/src/wit/mod.rs:482:52:
no entry found for key
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
