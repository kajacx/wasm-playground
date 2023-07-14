#!/usr/bin/sh
set -e

sh build-plugins.sh

cd runtime-ts
yarn start
cd ..

echo "Done"
