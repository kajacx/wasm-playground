#!/usr/bin/sh

cd wasmer-plugin && \
cargo build --target=wasm32-unknown-unknown && \
cd .. && \
\
cd wasmer3-runtime && \
cargo run && \
cd .. && \
\
echo "All done"
