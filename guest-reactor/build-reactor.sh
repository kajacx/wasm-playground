#!/usr/bin/sh
set -e

# Run from the parten directory

cd guest-reactor
cargo component build --target=wasm32-unknown-unknown

cd target/wasm32-unknown-unknown/debug/
jco transpile guest_reactor.wasm --instantiation -o out-dir
