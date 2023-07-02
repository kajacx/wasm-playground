#!/usr/bin/sh
set -e

plugin-rust/build-plugin.sh

cd runtime-web-jco

cp -r ../plugin-rust/target/wasm32-unknown-unknown/debug/out-dir ./

docker-compose down
docker-compose up -d

echo "All done, view the webpage at http://127.0.0.1:8095"
