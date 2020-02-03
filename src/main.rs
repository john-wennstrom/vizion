extern crate skew;

use std::env;
use skew::unskew;

fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    let src = env::args().nth(1).unwrap();
    let dst = env::args().nth(2).unwrap();

    let _skew_lines = unskew(&src, &dst);
}
