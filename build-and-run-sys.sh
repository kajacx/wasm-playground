#!/usr/bin/sh
set -e

echo BUILDING GUEST
guest-rust/build-guest.sh

echo GUEST BUILT, RUNNING HOST

cd runtime-rust-wasmtime
cargo run

echo "All done."
