
const KERNEL: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]];

pub struct Path<'s> {
  src: &'s str,
  dst: &'s str
}

pub struct Conv2d {
  kernel: Vec<Vec<i32>>,
}

pub struct Runner<'s> {
  path: Path<'s>,
  image: Vec<Vec<[u8; 3]>>,
  convolution: Conv2d
}

pub type Kernel:  

impl<'s> Path<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Path<'s> {
    Path {
      src: src,
      dst: dst
    }
  }
}

impl Conv2d {
  pub fn new(kernel: Vec<Vec<i32>>) -> Conv2d {
    Conv2d {
      kernel: kernel
    }
  }
}

impl<'s> Runner<'s> {
  pub fn new() -> Runner<'s> {
    Runner {
      path: Path::new("", ""),
      image: vec![],
      convolution: Conv2d::new(KERNEL)
    }
  }
}

