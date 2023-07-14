#!/usr/bin/sh
set -e

sh build-plugins.sh

cd runtime-rust
cargo run
cd ..

echo "Done"
