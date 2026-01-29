#!/bin/bash

echo "[INFO] Build Whamm Core Library..."
pushd whamm_core
# rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
popd
