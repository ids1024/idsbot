#!/bin/bash

# The ~/.cargo/config is configured to use this file as a linker.
~/Devel/src/git/rpi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc -Wl,-rpath-link=$HOME/Devel/rpi/packages/extracted/usr/lib $@
