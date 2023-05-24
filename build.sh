#!/usr/bin/sh

cd simple-plugin && cargo build --target wasm32-unknown-unknown && cd .. && \
cd hello-wasmer-u64 && wasm-pack build --target web && \
cp pkg/hello_wasmer_u64_bg.wasm out/hello_wasmer_u64_bg.wasm && \
cp pkg/hello_wasmer_u64.js out/hello_wasmer_u64.js && \
cd out && docker-compose -p wasmer-u64 down && docker-compose -p wasmer-u64 up -d && \
cd ../.. && \
echo "All done, view the webpage at http://127.0.0.1:8094"
