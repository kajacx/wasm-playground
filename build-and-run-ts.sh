#!/usr/bin/sh
set -e

cd plugin
cargo rustc --target wasm32-unknown-unknown -- -C target-feature=+multivalue
cd ..

cd runtime-ts
yarn start
cd ..

echo "Done"
