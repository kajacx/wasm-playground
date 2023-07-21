#!/usr/bin/sh
set -e

cargo build --target=wasm32-unknown-unknown

cd ../target/wasm32-unknown-unknown/debug
wasm-tools component new guest.wasm -o component.wasm
