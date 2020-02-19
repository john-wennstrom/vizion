
extern crate image;
use image::{RgbImage, ImageBuffer, Rgb};
use std::io;
use std::io::prelude::*;

const KERNEL: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]];

pub struct Path<'s> {
  src: &'s str
}

pub struct Conv2d {
  kernel: Vec<Vec<i32>>,
}

#[derive(Clone)]
pub struct Img {
  width: u32,
  height: u32,
  raw: Vec<u8>,
  pixels: Vec<Vec<[u8; 3]>>
}

pub struct Runner {
  img: Img,
  convolution: Conv2d
}

pub enum ConvolutionError {
  FromPath
}





impl<'s> Path<'s> {
  pub fn new(src: &'s str) -> Path<'s> {
    Path {
      src: src
    }
  }
}

impl Conv2d {
  pub fn new() -> Conv2d {
    let kernel = Conv2d::prepare_kernel();

    Conv2d {
      kernel: kernel
    }
  }

  fn prepare_kernel() -> Vec<Vec<i32>> {
    let mut kernel = vec![];

    for i in 0..KERNEL.len() {
      kernel.push(vec![]);
      for e in 0..KERNEL[i].len() {
        kernel[i].push(KERNEL[i][e]);
      }
    }

    kernel
  }
}

impl<'s> Img {
  pub fn new(src: & str) -> Img {
    let img = image::open(&src).expect("File not found").to_rgb();
    let (width, height) = img.dimensions();
    let raw = img.into_raw().clone();
    let pixels = Img::get_pixels(width, height, &raw);

    Img {
      width: width,
      height: height,
      raw: raw,
      pixels: pixels
    }
  }

  pub fn get_pixels(width: u32, height: u32, raw: &Vec<u8>) -> Vec<Vec<[u8; 3]>> {
    let mut pixels: Vec<Vec<[u8; 3]>> = vec![vec![]];

    let mut pixel: [u8; 3] = [0, 0, 0];
		let mut counter_pix: usize = 0; // Counter for R, G and B
    let mut counter_row: usize = 0; // Counter for rows
    
    for i in raw.clone() {

      // Add R/G/B value to pixel
      pixel[counter_pix] = i;

      // If pixel is full, append it to image (using c2 as row index) and  set
      // c1 as 0 to overwrite pixel RGB values
      if counter_pix == 2 {
        pixels[counter_row].push(pixel);
        counter_pix = 0;
      } else {

        // Increase c1
        counter_pix += 1;
      }

      // If the row has reached the end, and more rows can be pushed, increase c2 (and
      // push a new row) otherwise break
      if pixels[counter_row].len() as u32 == width {
        if (pixels.len() as u32) < height {
          pixels.push(vec![]);
          counter_row += 1;
        } else {
          break;
        }
      }
    }

    println!("{:?}", pixels);

    pixels
  }
}

impl Runner {
  pub fn new(img: Img, convolution: Conv2d) -> Runner {
    Runner {
      img: img,
      convolution: convolution
    }
  }

}

