#!/usr/bin/sh
set -e

cd plugin
cargo rustc --target wasm32-unknown-unknown -- -C target-feature=+multivalue
cd ..

cd runtime
cargo run
cd ..

echo "Done"
