extern crate unskew;

use std::env;
use unskew::Unskew;

fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    let src = env::args().nth(1).unwrap();
    let dst = env::args().nth(2).unwrap();

    //let _skew_lines = unskew(&src, &dst);

    let image = Unskew::new(&src, &dst)
      .grayscale()
      .invert()
      .pad()
      .unskew()
      .save();

    println!("1: {:?}", image);

}
