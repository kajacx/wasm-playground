#!/usr/bin/sh

echo "Genrating protocol bindings..." && \
cd fp1-protocol && \
rm -rf bindings && \
cargo run && \
cd bindings/rust-plugin && \
cargo check && \
cd ../../.. && \
echo "Protocol bindings generated" && \
\
echo "Building WASM plugin..." && \
cd fp2-plugin && \
cargo build && \
cd .. && \
echo "WASM plugin built" && \
\
echo "Copying bindings files..." && \
#sed -i '/async/d' fp-protocol/bindings/rust-wasmer-runtime/bindings.rs && \
cp fp1-protocol/bindings/rust-wasmer-runtime/bindings.rs fp3-runtime/src/spec/bindings.rs && \
cp fp1-protocol/bindings/rust-wasmer-runtime/types.rs    fp3-runtime/src/spec/types.rs && \
echo "Binding types copied" && \
\
echo "Testing the runtime..." && \
cd fp3-runtime && \
cargo test && \
cargo run && \
cd .. && \
echo "Runtime tested" && \
\
echo "All done"
