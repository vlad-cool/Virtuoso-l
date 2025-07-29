#!/bin/bash

cd "$(dirname "$(readlink -f "$0")")"
cd ..

# rm -rf armbian_build
mkdir -p armbian_build
cd armbian_build
git clone https://github.com/armbian/build
cd build
git pull

cp -r ../../armbian/* .

bash bananapi_m2z.sh
