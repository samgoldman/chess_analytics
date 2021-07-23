#!/bin/bash
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
mkdir build
cd build
cmake .. -G Ninja -DCMAKE_INSTALL_PREFIX=./ -DFLATBUFFERS_BUILD_SHAREDLIB=ON
ninja
ninja install
sudo cp flatc /usr/bin
cd