#!/usr/bin/sh
set -e

cd guest
cargo component build --target wasm32-unknown-unknown
cd ..

cd host
cargo run
