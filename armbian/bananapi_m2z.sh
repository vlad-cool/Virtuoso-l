#!/bin/sh
./compile.sh \
BOARD=bananapim2zero \
BRANCH=current \
RELEASE=noble \
BSPFREEZE=yes \
BUILD_KSRC=yes \
KERNEL_CONFIGURE=no \
BUILD_DESKTOP=no \
BUILD_MINIMAL=no \
FIXED_IMAGE_SIZE=4096
