#!/usr/bin/sh
cd plugin-rust && cargo build --target=wasm32-unknown-unknown && cd target/wasm32-unknown-unknown/debug/ && \
wasm-tools component new plugin_rust.wasm -o component.wasm && jco transpile component.wasm --instantiation -o out-dir && cd ../../../../ && \
rm -rf runtime-web-jco/out-dir && mv plugin-rust/target/wasm32-unknown-unknown/debug/out-dir runtime-web-jco/ &&
cd runtime-web-jco && docker-compose down && docker-compose up -d && \
echo "All done, view the webpage at http://127.0.0.1:8095"
