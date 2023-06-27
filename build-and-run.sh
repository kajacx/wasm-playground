#!/usr/bin/sh
mkdir -p plugin-rust/wit && cp protocol.wit plugin-rust/wit/protocol.wit && \
cd plugin-rust && cargo build --target=wasm32-unknown-unknown && cd .. && \
mkdir -p runtime-rust-wasmtime/wit && cp protocol.wit runtime-rust-wasmtime/wit/protocol.wit && \
cd runtime-rust-wasmtime && cargo run && cd .. && \
echo "All done."
