
mod matrix;

use simple_matrix::Matrix;
use matrix::conv::Convolution;

pub struct Convolve<'s> {
    src_path: &'s str,
    dst_path: &'s str,
}

impl<'s> Convolve<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Convolve<'s> {
    Convolve {
      src_path: src,
      dst_path: dst
    }
  }

  pub fn convolve(self) -> Convolve<'s> {
    let mat: Matrix<i32> = Matrix::new(5, 5);
    let inp_mat: Matrix<i32> = Matrix::new(5, 5);

    mat.conv(inp_mat);

    self
  }
}
