#!/usr/bin/sh
set -e

guest-rust/build-guest.sh

cd runtime-rust-wasmtime
cargo run

echo "All done."
