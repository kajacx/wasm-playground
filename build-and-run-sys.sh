#!/usr/bin/sh
set -e

guest-rust/build-guest.sh

echo GUEST BUILD, RUNNING HOST

cd runtime-rust-wasmtime
cargo run

echo "All done."
