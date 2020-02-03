# unswirl

### Requirements
* Opencv
* Opencv_contrib

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
```
target/debug/unswirl examples/src1.png examples/dst1.png
target/debug/unswirl examples/src2.png examples/dst2.png
```

### TODO
* Add border before finding the box with copyMakeBorder using BORDER_CONSTANT with background color
* Refactor to functional programming

Make composition
```
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

Then we can use it like

let add = | x: i32 | x + 2;
let multiply = | x: i32 | x * 2;
let divide = | x: i32 | x / 2;

let intermediate = compose!(add, multiply, divide);

let subtract = | x: i32 | x - 1;

let finally = compose!(intermediate, subtract);

let expected = 11;
let result = finally(10);
assert_eq!(result, expected); // passes
```
