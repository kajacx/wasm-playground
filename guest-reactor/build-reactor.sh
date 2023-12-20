#!/usr/bin/sh
set -e

# Run from the parten directory

cd guest-reactor
cargo component build --release --target=wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/release/
jco transpile guest_reactor.wasm --instantiation -o out-dir

wasm-tools print guest_reactor.wasm > guest_reactor.wat
wasm-tools print out-dir/guest_reactor.core.wasm > out-dir/guest_reactor.core.wat
wasm-tools print out-dir/guest_reactor.core2.wasm > out-dir/guest_reactor.core2.wat
wasm-tools print out-dir/guest_reactor.core3.wasm > out-dir/guest_reactor.core3.wat
