## Test contracts

This contracts is provided for testing purpose only.


### How to compile:
```
cargo +nightly build --target wasm32-unknown-unknown --release -Z unstable-options --out-dir ./wasm
wasm-opt -Os -o wasm/test_contract.wasm wasm/test_contract.wasm
```