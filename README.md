# unswirl

### Requirements
* Opencv
* Opencv_contrib

### Build OpenCV
Make sure .env file is configured, then run
```
./prebuild.sh
```

### Build and run unswirl
Make sure .env file is configured, then run
```
./build.sh
```

Run examples
```
target/debug/vizion unskew examples/src1.png examples/src1_result.png
target/debug/vizion face-detection examples/people.jpg examples/people_result.png
target/debug/vizion convolution examples/dependable.png examples/dependable_result.png
```
