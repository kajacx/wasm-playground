#!/usr/bin/sh
set -e

sh plugin-wasi/build-plugin.sh

cd runtime-rust-wasmtime-sync
echo "CMD LINE" | cargo run

echo "All done."
