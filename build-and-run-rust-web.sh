#!/usr/bin/sh
cd plugin-rust && cargo build --target=wasm32-unknown-unknown && cd target/wasm32-unknown-unknown/debug/ && \
wasm-tools component new plugin_rust.wasm -o component.wasm && jco transpile component.wasm --instantiation -o out-dir && cd ../../../../ && \
rm -rf runtime-rust-web/rust-web/out-dir && mv plugin-rust/target/wasm32-unknown-unknown/debug/out-dir runtime-rust-web/rust-web &&
cd runtime-rust-web && wasm-pack build --target web && rm -rf rust-web/pkg && cp -r pkg rust-web && \
cd rust-web && docker-compose down && docker-compose up -d && cd ../.. && \
echo "All done, view the webpage at http://127.0.0.1:8096"
