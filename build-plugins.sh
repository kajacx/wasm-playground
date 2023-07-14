#!/usr/bin/sh
set -e

cd plugin
cargo rustc --target wasm32-unknown-unknown -- -C target-feature=+multivalue
cd ..

cd plugin-wasi
# cargo +nightly rustc -Zbuild-std=std,panic_abort -Zmultitarget --target wasm32-wasi -- -C target-feature=+multivalue
cargo build --target wasm32-wasi
cd ..
