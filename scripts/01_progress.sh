#!/bin/bash

echo "[INFO] Installing 'progress'..."
git clone --depth=1 https://github.com/titzer/progress.git
cp -fv  progress/bin/progress.x86-64-linux $HOME/.local/bin
rm -rf progress