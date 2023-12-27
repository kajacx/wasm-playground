#!/usr/bin/sh
set -e

cd guest-no-imports
cargo component build --release
cd ..

cd host-no-imports
cargo run
cd ..

echo "All done."
