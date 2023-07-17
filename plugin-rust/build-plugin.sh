#!/usr/bin/sh
set -e

# Run from the parten directory

cd plugin-rust
cargo build --target=wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/debug/
wasm-tools component new plugin_rust.wasm -o component.wasm
jco transpile component.wasm --instantiation -o out-dir
jco transpile component.wasm -o no-instance
