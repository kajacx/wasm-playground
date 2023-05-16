#!/usr/bin/sh

cd plugin && cargo build --release --target wasm32-unknown-unknown && cd ..  && \
cd no-wai-web && \
wasm-pack build --target web && \
# wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/debug/bevy-web-test.wasm && \
cp pkg.gitignore pkg/.gitignore
cd pkg && docker-compose -p forked down && docker-compose -p forked up -d && cd .. && \
cd .. && \
echo "Done, view at http://localhost:8093"
