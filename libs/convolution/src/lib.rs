
extern crate image;
extern crate imagefmt;


use std::cmp;
use std::time::{Instant};

const KERNEL: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]];
const SHADE: u8 = 0;

#[allow(dead_code)]
pub struct Path<'s> {
  src: &'s str
}

#[derive(Clone)]
pub struct Img {
  width: u32,
  height: u32,
  raw: Vec<u8>,
  pixels: Vec<Vec<u8>>
}

#[derive(Clone)]
pub struct Conv2d {
  img: Img,
  kernel: Vec<Vec<i32>>,
  result: Vec<Vec<u8>>
}

#[allow(dead_code)]
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
  pub fn new(img: Img) -> Conv2d {
    let kernel = Conv2d::prepare_kernel();

    Conv2d {
      img: img,
      kernel: kernel,
      result: vec![]
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

  pub fn run(self) -> Vec<Vec<u8>> {
    let now = Instant::now();

    let width = self.img.width as usize;
    let height = self.img.height as usize;
    let klen = self.kernel.len();
    let constrain = |x: i32| { cmp::max(0, cmp::min(255 as i32, x)) };

    // Border width used for padding
    let w = (klen - 1) / 2;

    let mut result: Vec<Vec<u8>> = vec![vec![0; width - (klen - 1)]; height - (klen - 1)];
    let mut i = 0;

    // Loop rows in image
    for row in w..(height - w) {

      //let mut current_row: Vec<u8> = vec![];
      let mut j = 0;

      // Loop pixels in row
      for pixel in w..(width - w) {

        // Calculated pixel
        let mut calculated_pixel: i32 = 0;

        // Apply kernel
        // Loop kernel rows
        for krow in 0..klen {

          // Loop pixels in kernel row
          for kpixel in 0..klen {
            let row_offset = row + krow - w;
            let col_offset = pixel + kpixel - w;
            calculated_pixel += self.img.pixels[row_offset][col_offset] as i32 * self.kernel[krow][kpixel];
          }
        }

        calculated_pixel = constrain(calculated_pixel);
        result[i][j] = calculated_pixel as u8;

        j = j + 1;
      }

      //result[i] = current_row;
      i = i + 1;
    }

    //println!("i: {}, height: {}", i, height);

    println!("3: {} ms", now.elapsed().as_micros());

    result
  }
}

impl<'s> Img {
  pub fn new(src: & str) -> Img {
    let img = image::open(&src).expect("File not found").to_luma();
    let (width, height) = img.dimensions();
    let raw = img.into_raw().clone();
    let pixels = Img::get_pixels_gray(width, height, &raw);

    Img {
      width: width,
      height: height,
      raw: raw,
      pixels: pixels
    }
  }

  pub fn get_pixels_gray(width: u32, height: u32, raw: &Vec<u8>) -> Vec<Vec<u8>> {
    let w = width as usize;
    let h = height as usize;

    let mut matrix: Vec<Vec<u8>> = vec![];

    for i in 0..h {
      let row = &raw[(i * w)..((i * w) + w)].to_vec();
      matrix.push(row.clone());
    }

    matrix
  }

  pub fn add_border(mut self, kernel_length: usize) -> Img {
    let w = (kernel_length - 1) / 2;

    // Top / Bottom border
    for _ in 0..w {
      self.pixels.insert(0, vec![SHADE; self.width as usize]);
      self.pixels.push(vec![SHADE; self.width as usize]);
    }

    let mut matrix: Vec<Vec<u8>> = vec![];

    // Left / Right borders
    for mut row in self.pixels.clone() {
      for _ in 0..w {
        row.insert(0, SHADE);
        row.push(SHADE);
      }

      matrix.push(row);
    }

    self.pixels = matrix;
    self.width += (w * 2) as u32;
    self.height += (w * 2) as u32;
    self
  }
}

impl Runner {
  pub fn new(mut img: Img, convolution: Conv2d) -> Runner {
    img = img.add_border(convolution.kernel.len());
    
    Runner {
      img: img,
      convolution: convolution
    }
  }

  pub fn run(mut self) -> Runner {
    let convolution = self.convolution.clone();
    self.convolution.result = convolution.run();
    self
  }

  pub fn save(self, name: &str) -> bool {
    let mut result: Vec<u8> = vec![];

    for row in self.convolution.result.clone() {
      for pixel in row {
        result.push(pixel);
        result.push(pixel);
        result.push(pixel);
      }
    }

    let result = imagefmt::write(
      name,
      self.convolution.result[0].len(), // width
      self.convolution.result.len(),    // height
      imagefmt::ColFmt::RGB,
      &result[..],
      imagefmt::ColType::Gray
    );
    
    match result {
      Ok(_) => true,
      Err(_) => false
    }
  }

}

