#!/usr/bin/env nix-shell
#!nix-shell shell.nix --attr xgboost -i bash

if [ ! -d xgboost/ ]; then
    git clone --recursive https://github.com/dmlc/xgboost
fi
cd xgboost/ || exit
make -j4
cd ../ || exit
