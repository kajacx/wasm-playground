#!/usr/bin/sh

cd plugin && cargo build --target wasm32-unknown-unknown && cd .. && \
cd runtime && cargo run && cd .. && \
echo "Done"
