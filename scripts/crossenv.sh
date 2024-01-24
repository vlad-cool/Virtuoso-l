#!/bin/sh

PYTHON_VERSION=3.12

cd python
git clone https://github.com/python/cpython --branch $PYTHON_VERSION
cd python/cpython
./configure
make -s -j8