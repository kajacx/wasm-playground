#!/usr/bin/sh

rm -rf host-bindings && mkdir host-bindings && \
cd host-bindings && wasmer run wasmer/wai-bindgen-cli --dir ../.. -- js --export ../../protocol-host.wai && cd .. && \
\
rm -rf guest-bindings && mkdir guest-bindings && \
cd guest-bindings && wasmer run wasmer/wai-bindgen-cli --dir ../.. -- js --import ../../protocol-plugin.wai && cd .. && \
\
yarn install && yarn start && \
true
