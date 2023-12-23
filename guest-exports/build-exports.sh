#!/usr/bin/sh
set -e

# Run from the parten directory

cd guest-exports
cargo component build --release --target=wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/release/
jco transpile guest_exports.wasm --instantiation -o out-dir

wasm-tools print guest_exports.wasm > guest_exports.wat
wasm-tools print out-dir/guest_exports.core.wasm > out-dir/guest_exports.core.wat
wasm-tools print out-dir/guest_exports.core2.wasm > out-dir/guest_exports.core2.wat
wasm-tools print out-dir/guest_exports.core3.wasm > out-dir/guest_exports.core3.wat
