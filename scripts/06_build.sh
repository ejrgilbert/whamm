#!/bin/bash

echo "[INFO] Build"
cargo build

echo "[INFO] Run fmt"
cargo fmt --all
cargo fmt --all -- --check

echo "[INFO] Run tests"
cargo test --verbose -- --nocapture
