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
EXTRA_PACKAGES_IMAGE="sway mingetty overlayroot" \
EXTRA_PACKAGES_IMAGE_REFS="build:compile.sh:0 build:compile.sh:0 build:compile.sh:0" \
FIXED_IMAGE_SIZE=4096
