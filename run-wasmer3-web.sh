#!/usr/bin/sh

cd wasmer-plugin && \
cargo build --target=wasm32-unknown-unknown && \
cd .. && \
\
cd wasmer3-runtime && \
wasm-pack build --target web && \
# wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/bevy-web-test.wasm && \
cd pkg && docker-compose -p wasmer3-web down && docker-compose -p wasmer3-web up -d && cd .. && \
cd .. && \
echo "Done, view at http://localhost:8093"
