extern crate skew;
extern crate image;
extern crate imageproc;

use std::env;
use std::path::Path;
use std::fs;
use image::{open};
use opencv::prelude::*;
use skew::determine_skew;
use imageproc::hough::{PolarLine};

fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    let input_path = env::args().nth(1).unwrap();
    let output_dir = env::args().nth(2).unwrap();

    let input_path = Path::new(&input_path);
    let output_dir = Path::new(&output_dir);

    if !output_dir.is_dir() {
        fs::create_dir(output_dir).expect("Failed to create output directory")
    }

    if !input_path.is_file() {
        panic!("Input file does not exist");
    }

    // Load image and convert to grayscale
    let image = open(input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_luma();

    let skew_lines: Vec<PolarLine> = determine_skew(&image, &output_dir);


    for (i, line) in skew_lines.iter().enumerate() {
        if i % 10 == 0 {
            println!("{:?}, {:?}", line.angle_in_degrees, line.r);
        }
    }
}
