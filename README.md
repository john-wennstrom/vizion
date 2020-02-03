# unswirl

### Requirements
* Opencv
* Opencv_contrib

### Future requirements
* rustacuda
* iamgeproc

### Build OpenCV
```
mkdir build && cd build
cmake -D CMAKE_BUILD_TYPE=Release -D OPENCV_GENERATE_PKGCONFIG=ON -D CMAKE_INSTALL_PREFIX=/usr/local -D OPENCV_EXTRA_MODULES_PATH=<opencv_contrib>/modules <opencv_src_path>

make -j6
```

### Build and run unswirl
Export environment variable for package config files, where opencv4.pc exist
```export PKG_CONFIG_PATH=[path]```

Configure
```pkg-config --cflags opencv4```

Build
```cargo build```

Run
```target/debug/unswirl examples/test.png examples/test1.png```
