#!/usr/bin/sh
set -e

# Run from the parten directory

cd guest-rust
# cargo component build --release --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/release/
# jco transpile guest_rust.wasm --instantiation -o out-dir
# jco transpile guest_rust.wasm -o no-instance
