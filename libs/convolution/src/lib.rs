
extern crate image;

mod matrix;

use std::cmp;
use image::{DynamicImage, GenericImageView, Luma};
use simple_matrix::{Matrix};

static KERNEL: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]]; 

#[derive(Clone)]
pub struct Convolve<'s> {
    src_path: &'s str,
    dst_path: &'s str,
    img: DynamicImage,
    mat: Matrix<u8>,
    pixels: Vec<u8>
}

impl<'s> Convolve<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Convolve<'s> {
    Convolve {
      src_path: src,
      dst_path: dst,
      img: image::open(&src).unwrap(),
      mat: Matrix::new(2, 2),
      pixels: Vec::new()
    }
  }

  pub fn convolve(&mut self) -> Result<&'static str, &'static str> {
    self.img = self.img.grayscale();
    let pixels = self.img.to_bytes();
    let (width, height) = self.img.dimensions();

    self.mat = Matrix::from_iter(width as usize, height as usize, pixels.clone());
   
    let mut color = Luma([0]);
    let rows = self.mat.rows() as i32;
    let cols = self.mat.cols() as i32;

    println!("{:?},{:?}", rows,cols);
    
    for x in 0..rows {
      for y in 0..cols {
        color = Convolve::convolution(x, y, rows, pixels.clone());
        //println!("{:?}", color);
      }
    }

    Ok("Success")
  }

  fn convolution(x: i32, y: i32, width: i32, pixels: Vec<u8>) -> Luma<u8> {
    let matrix_size = 3;
    let offset = matrix_size / 2;
    let mut total = 0i32;

    // Loop through kernel matrix
    for i in 0..matrix_size as usize {
      for j in 0..matrix_size as usize {

        // Pixel to test
        let xloc = x + i as i32 - offset;
        let yloc = y + j as i32 - offset;
        let mut loc = xloc + width * yloc;
        //println!("Loc: {:?}", loc);

        loc = Convolve::constrain(loc, 0, pixels.len() - 1);
        //println!("Loc1: {:?}", loc);

        let pixel = KERNEL[i][j];
        total += (pixels[loc as usize] as i32) * pixel as i32;
      }
    }
    println!("{:?}", total);
    Luma([0])
  }

  fn constrain(val: i32, lower: i32, upper: usize) -> i32 {
    cmp::max(lower, cmp::min(upper as i32, val))
  }
}
