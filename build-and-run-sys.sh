#!/usr/bin/sh
set -e

plugin-rust/build-plugin.sh

cd runtime-rust-wasmtime
cargo run

echo "All done."
