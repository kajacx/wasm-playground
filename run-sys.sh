#!/usr/bin/sh
set -e

cd guest
sh build.sh
cd ..

cd host-sys
cargo run
