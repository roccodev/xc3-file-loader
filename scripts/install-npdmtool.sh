#!/bin/bash
set -e

apt install -y git

git clone https://github.com/switchbrew/switch-tools --depth=1 --branch=v1.13.1
cd switch-tools
./autogen.sh
./configure
make install