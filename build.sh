#!/bin/bash

# Import .env variables
set -o allexport
source .env
set +o allexport

# Configure
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH
pkg-config --cflags opencv4

# Build
cargo build
