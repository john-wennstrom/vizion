# unswirl

### Requirements
* OpenCV

### Build OpenCV
```
cmake -D CMAKE_BUILD_TYPE=Release -D OPENCV_GENERATE_PKGCONFIG=ON -D CMAKE_INSTALL_PREFIX=/usr/local [opencv src]
make -j6
```

### Build unswirl
Export environment variable for package config files, where opencv4.pc exist
`export PKG_CONFIG_PATH=[path]`

Configure
`pkg-config --cflags opencv4`

Build
`cargo build`
