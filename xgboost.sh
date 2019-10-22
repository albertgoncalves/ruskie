#!/usr/bin/env nix-shell
#!nix-shell xgboost.nix -i bash

if [ ! -d xgboost/ ]; then
    git clone --recursive https://github.com/dmlc/xgboost
fi
cd xgboost/ || exit
make -j4
cd ../ || exit
