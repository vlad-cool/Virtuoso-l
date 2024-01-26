#!/bin/sh

PYTHON_VERSION=3.12

HOST_PYTHON=$(pwd)/host_python
BUILD_PYTHON=$(pwd)/build_python

# git clone https://github.com/python/cpython --branch $PYTHON_VERSION
cd cpython

./configure --prefix=$HOST_PYTHON --enable-optimizations
./configure --prefix=$BUILD_PYTHON
make -j4
make install

ARCH=arm
CC=arm-linux-gnu-gcc
AR=arm-linux-gnu-ar

./configure \
    --build=x86_64-pc-linux-gnu \
    --host=arm-linux-gnueabihf \
    --prefix=$HOST_PYTHON \
    --with-build-python=$BUILD_PYTHON/bin/python3 \
    --disable-ipv6 \
    ac_cv_buggy_getaddrinfo=no \
    ac_cv_file__dev_ptmx=yes \
    ac_cv_file__dev_ptc=no

./configure \
    --build=x86_64-linux-gnu \
    --host=arm-linux-gnu \
    --disable-ipv6 \
    ac_cv_buggy_getaddrinfo=no \
    ac_cv_file__dev_ptmx=yes \
    ac_cv_file__dev_ptc=no \

make -j8
make install

cd ..

./build_python/bin/python3 -m pip install crossenv
./build_python/bin/python3 -m crossenv ./host_python/bin/python3 venv