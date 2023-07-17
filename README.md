# Using wit bindgen without wasi

This is a proof-of-concept project that showcased how to use wit bindgen in Rust and on the web.

## Rust plugin

Inspect the [plugin-rust](/plugin-rust/) folder,
and see the [`build-plugin.sh`](/plugin-rust/build-plugin.sh) script on how to build the plugin.

## Rust wasmtime runtime

Inspect the [runtime-rust-wasmtime](/runtime-rust-wasmtime/) folder,
and see the [`build-and-run-sys.sh`](/build-and-run-sys.sh) script on how to run the project.

## Web runtime with jco

The `jco transpile component.wasm --instantiation -o out-dir` command prepares the component so that it can run on the web.

Inspect the [runtime-web-jco](/runtime-web-jco/) folder,
and see the [`build-and-run-web.sh`](/build-and-run-web.sh) script on how to run the project.

Docker is used to host the website, but you can host it in any way you like.

## Web runtime in Rust

A "minimal viable example" project on how to run the component on the web from Rust code.

Use [wasm-bridge](https://github.com/kajacx/wasm-bridge) if you want to load and run WASM components on the web from Rust.

If you want to "peek behind the curtain" and learn a little bit how wasm-bridge works, you can
inspect the [runtime-rust-web](/runtime-rust-web/) folder,
and see the [`build-and-run-rust-web.sh`](/build-and-run-rust-web.sh) script on how to run the project.