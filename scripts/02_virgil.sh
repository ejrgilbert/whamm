#!/bin/bash

echo "[INFO] Building 'virgil'..."
git clone --depth=1 https://github.com/titzer/virgil.git
pushd virgil
export PATH=$PATH:$(pwd)/bin
make
echo "[INFO] Testing 'virgil' is on PATH..."
which v3c
popd
cp -frv  virgil/bin $HOME/.local/bin
