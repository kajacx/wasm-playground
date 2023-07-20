#!/usr/bin/sh
set -e

guest-rust/build-guest.sh

cd runtime-rust-web

rm -rf rust-web/out-dir
cp -r ../guest-rust/target/wasm32-unknown-unknown/debug/out-dir rust-web/

wasm-pack build --target web
rm -rf rust-web/pkg
cp -r pkg rust-web/

cd rust-web
docker-compose down
docker-compose up -d

echo "All done, view the webpage at http://127.0.0.1:8096"
