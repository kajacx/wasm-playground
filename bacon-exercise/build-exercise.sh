#!/usr/bin/sh
set -e

# Run from parent directory

# Build wasm32-wasi
cd bacon-exercise
cargo component build --release
