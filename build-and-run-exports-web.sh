#!/usr/bin/sh
set -e

guest-exports/build-exports.sh

cd runtime-web-jco

cp -r ../guest-exports/target/wasm32-unknown-unknown/release/out-dir ./

docker-compose down
docker-compose up -d

echo "All done, view the webpage at http://127.0.0.1:8095"
