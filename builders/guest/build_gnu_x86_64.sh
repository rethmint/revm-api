#!/bin/bash
set -o errexit -o nounset -o pipefail

export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse


echo "Starting x86_64-unknown-linux-gnu build"
export CC=clang
export CXX=clang++
cargo build --release --target x86_64-unknown-linux-gnu
cp "$./target/x86_64-unknown-linux-gnu/release/librevmapi.so" artifacts/librevmapi.x86_64.so
