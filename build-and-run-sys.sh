#!/usr/bin/sh
cd plugin-rust && cargo build --target=wasm32-unknown-unknown && \
cd target/wasm32-unknown-unknown/debug && wasm-tools component new plugin_rust.wasm -o component.wasm && cd ../../../.. && \
cd runtime-rust-wasmtime && cargo run && cd .. && \
echo "All done."
