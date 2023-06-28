#!/usr/bin/sh
cd plugin-rust && cargo build --target=wasm32-unknown-unknown && cargo build --target=wasm32-wasi && cd .. && \
mkdir -p runtime-rust-wasmtime/wit && cp protocol.wit runtime-rust-wasmtime/wit/protocol.wit && \
echo "All done."
