#!/bin/bash

set -o allexport
source .env
set +o allexport

OPENCV_CONTRIB_PATH=$PROJECT_PATH/vendor/opencv_contrib
CMAKE_INSTALL_PREFIX=/usr/local

mkdir -p vendor
cd vendor

git clone https://github.com/opencv/opencv.git
git clone https://github.com/opencv/opencv_contrib.git

cd opencv

sudo add-apt-repository "deb http://security.ubuntu.com/ubuntu xenial-security main"
sudo apt-get update
sudo apt-get upgrade

sudo apt install -y build-essential cmake unzip pkg-config
sudo apt install -y libjpeg-dev libpng-dev libtiff-dev
sudo apt install -y libjasper1 libjasper-dev
sudo apt install -y libavcodec-dev libavformat-dev libswscale-dev libv4l-dev
sudo apt install -y libxvidcore-dev libx264-dev
sudo apt install -y libgtk-3-dev
sudo apt install -y libatlas-base-dev gfortran
sudo apt install -y libopencv-dev

mkdir -p build
cd build
cmake -D CMAKE_BUILD_TYPE=Release -D OPENCV_GENERATE_PKGCONFIG=ON -D CMAKE_INSTALL_PREFIX=$CMAKE_INSTALL_PREFIX -D OPENCV_EXTRA_MODULES_PATH=$OPENCV_CONTRIB_PATH/modules ..
make -j $(nproc)
sudo make install