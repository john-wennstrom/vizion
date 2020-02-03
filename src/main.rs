extern crate skew;
extern crate image;

use std::env;
use skew::unskew;

fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    let src = env::args().nth(1).unwrap();
    let dst = env::args().nth(2).unwrap();

    /*let src = Path::new(&src);
    let dst = Path::new(&dst);

    if !dst.is_dir() {
        fs::create_dir(dst).expect("Failed to create output directory")
    }*/

    /*if !src.is_file() {
        panic!("Input file does not exist");
    }*/

    let _skew_lines = unskew(&src, &dst);

    /*for (i, line) in skew_lines.iter().enumerate() {
        if i % 10 == 0 {
            println!("{:?}, {:?}", line.angle_in_degrees, line.r);
        }
    }*/
}
