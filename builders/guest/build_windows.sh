#!/bin/bash
set -o errexit -o nounset -o pipefail

export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# ref: https://www.reddit.com/r/rust/comments/5k8uab/crosscompiling_from_ubuntu_to_windows_with_rustup/

cargo build --release --target-dir="./target" --target x86_64-pc-windows-gnu

cp "./target/x86_64-pc-windows-gnu/release/librevmapi.dll" artifacts/librevmapi.dll
