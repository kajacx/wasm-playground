#!/usr/bin/sh
set -e

rm -rf wasi-dir/wasi_*
rm -rf wasi-dir/interfaces/*

cargo run

cd wit-dir
wasm-tools print wit_components_guest.core.wasm > wit_components_guest.core.wat 
wasm-tools print wit_components_guest.core2.wasm > wit_components_guest.core2.wat 
wasm-tools print wit_components_guest.core3.wasm > wit_components_guest.core3.wat 
cd ..

cd wasi-dir
wasm-tools print wasi_components_guest.core.wasm > wasi_components_guest.core.wat 
wasm-tools print wasi_components_guest.core2.wasm > wasi_components_guest.core2.wat 
wasm-tools print wasi_components_guest.core3.wasm > wasi_components_guest.core3.wat 
wasm-tools print wasi_components_guest.core4.wasm > wasi_components_guest.core4.wat 
cd ..
