#!/usr/bin/sh
set -e

sh plugin-wasi/build-plugin.sh

jco transpile plugin-wasi/target/wasm32-wasi/release/plugin_wasi.wasm --instantiation -o runtime-web-jco/out-dir/


cd runtime-web-jco

docker compose down
docker compose up -d

echo "All done, view the webpage at http://127.0.0.1:8099"
