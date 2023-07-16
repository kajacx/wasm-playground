#!/usr/bin/sh
set -e

# Build wasm32-wasi
cd plugin-wasi
cargo component build
