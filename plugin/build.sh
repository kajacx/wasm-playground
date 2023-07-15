#!/usr/bin/sh
set -e

cargo build --target=wasm32-unknown-unknown

cd ../target/wasm32-unknown-unknown/debug
wasm-tools component new plugin.wasm -o component.wasm
jco transpile component.wasm --instantiation -o out-dir
wasm-bridge-cli out-dir -u component.wasm -o component.zip
