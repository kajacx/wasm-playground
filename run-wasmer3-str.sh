#!/usr/bin/sh

cd wasmer-plugin-str && \
cargo build --target=wasm32-unknown-unknown && \
cd .. && \
\
cd wasmer3-runtime-str && \
cargo run && \
cd .. && \
\
echo "All done"
