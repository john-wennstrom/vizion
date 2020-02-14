
extern crate image;

mod matrix;

use image::{DynamicImage, GenericImageView};
use simple_matrix::{Matrix};
use matrix::conv::Convolution;

pub struct Convolve<'s> {
    src_path: &'s str,
    dst_path: &'s str,
    img: DynamicImage,
    mat: Matrix<u8>
}

impl<'s> Convolve<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Convolve<'s> {
    Convolve {
      src_path: src,
      dst_path: dst,
      img: image::open(&src).unwrap(),
      mat: Matrix::new(2, 2)
    }
  }

  pub fn convolve(mut self) -> Convolve<'s> {
    self.img = self.img.grayscale();
    let pixel_vec: Vec<u8> = self.img.to_bytes();
    let (width, height) = self.img.dimensions();

    let mat: Matrix<u8> = Matrix::from_iter(width as usize, height as usize, pixel_vec);

    let inp_mat: Matrix<u8> = Matrix::new(5, 5);

    //self.mat = mat.conv(inp_mat);

    println!("{:?}", mat);

    self
  }
}
