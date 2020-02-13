
mod matrix;

pub struct Convolution<'s> {
    src_path: &'s str,
    dst_path: &'s str,
}

impl<'s> Convolution<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Convolution<'s> {
    Convolution {
      src_path: src,
      dst_path: dst
    }
  }
}
