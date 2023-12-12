#!/usr/bin/sh
set -e

# Run from the parten directory

cd guest-reactor
cargo component build --target=wasm32-unknown-unknown

