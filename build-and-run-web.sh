#!/usr/bin/sh
set -e

sh plugin-wasi/build-plugin.sh

cp plugin-wasi/target/wasm32-wasi/release/plugin_wasi.wasm  runtime-web-rollup/out-dir/

cd runtime-web-rollup
docker compose down
docker compose up -d

echo "All done, view the webpage at http://127.0.0.1:8098"
