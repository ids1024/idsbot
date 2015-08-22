#!/bin/bash

# Needed for cross compiling rust-openssl
export CC=~/Devel/src/git/rpi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc
export OPENSSL_LIB_DIR=~/Devel/rpi/packages/extracted/usr/lib
export OPENSSL_INCLUDE_DIR=~/Devel/rpi/packages/extracted/usr/include
#export LD=$PWD/ldwrap.sh

cargo build --release --target arm-unknown-linux-gnueabihf $@
