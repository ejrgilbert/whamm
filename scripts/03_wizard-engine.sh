#!/bin/bash

pushd virgil
export PATH=$PATH:$(pwd)/bin
popd
echo "[INFO] Building 'wizeng'..."
git clone --depth=1 https://github.com/titzer/wizard-engine.git
pushd wizard-engine
make -j
popd

echo "[INFO] Configuring the 'wizeng' spectest interpreter..."
mkdir -p output/tests/engines
mv wizard-engine/bin/unittest.x86-linux output/tests/engines/wizard-spectest

echo "[INFO] Testing the 'wizeng' spectest interpreter configuration..."
ls -al output/tests/engines
./output/tests/engines/wizard-spectest --help || [ $? == 1 ]

echo "[INFO] Configuring the 'wizeng' binary..."
ln -s ${PWD}/wizard-engine/scripts/wizeng output/tests/engines/wizeng

echo "[INFO] Testing the 'wizeng' binary configuration..."
./output/tests/engines/wizeng --help
 