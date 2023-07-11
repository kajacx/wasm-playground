#!/usr/bin/sh
set -e

cd plugin
sh build.sh
cd ..

cd host-sys
cargo run
