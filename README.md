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
