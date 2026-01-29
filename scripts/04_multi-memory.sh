#!/bin/bash

echo "[INFO] Configuring OCaml..."
opam init -y
eval $(opam env)
opam install -y dune
opam install -y menhir

echo "[INFO] Building the wasm ref interpreter..."
git clone --depth=1 https://github.com/WebAssembly/multi-memory.git
pushd multi-memory/interpreter
make
popd

echo "[INFO] Configuring the wasm ref interpreter..."
mv multi-memory/interpreter/wasm output/tests/engines

echo "[INFO] Testing the wasm ref interpreter configuration..."
./output/tests/engines/wasm --help
          