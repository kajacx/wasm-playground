#!/usr/bin/sh
set -e

cd plugin
sh build.sh
cd ..

cd host-js
wasm-pack build --target=web
rm -rf example-webserver/pkg
cp -r pkg example-webserver/

cd example-webserver
docker-compose down
docker-compose up -d

echo "Done, view the webpage at http://127.0.0.1:8097"
