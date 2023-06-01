#!/usr/bin/sh

cd rust-plugin && cargo build --target=wasm32-unknown-unknown && cd .. && \
cd rust-wasmer-host && cargo run && cd .. && \
echo All done
