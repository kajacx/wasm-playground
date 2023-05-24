#!/usr/bin/sh

cd plugin && cargo build --target wasm32-unknown-unknown && cd .. && \
cd host-wasmtime && cargo run

