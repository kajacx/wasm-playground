#!/usr/bin/sh
set -e

cd guest-imports
cargo component build --release
cd ..

cd host-imports
cargo run
cd ..

echo "All done."
