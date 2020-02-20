
extern crate image;

const KERNEL: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]];
const SHADE: u8 = 0;

#[allow(dead_code)]
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
  pixels: Vec<Vec<u8>>
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
    self
  }
}

impl Runner {
  pub fn new(mut img: Img, convolution: Conv2d) -> Runner {
    img = img.add_border(convolution.kernel.len());

    println!("{:?}", img.pixels);
    Runner {
      img: img,
      convolution: convolution
    }
  }

}

