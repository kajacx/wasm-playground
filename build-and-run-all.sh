#!/usr/bin/sh

cd rust-plugin && cargo build --target=wasm32-unknown-unknown && cd .. && \
cd rust-wasmer-host && cargo run && cd .. && \
cd ts-runtime && sh run.sh && cd .. && \
echo All done
