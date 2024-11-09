#!/bin/bash
set -o errexit -o nounset -o pipefail
mkdir -p artifacts

export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# No stripping implemented (see https://github.com/CosmWasm/wasmvm/issues/222#issuecomment-2260007943).

echo "Starting x86_64-unknown-linux-gnu build"
export CC=clang
export CXX=clang++
(cd librevm && cargo build --release --target x86_64-unknown-linux-gnu)
cp "./target/x86_64-unknown-linux-gnu/release/librevmapi.so" artifacts/librevmapi.x86_64.so
