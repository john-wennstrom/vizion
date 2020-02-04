extern crate opencv;
extern crate cast;

use opencv::{
    core as cv,
    prelude::*,
    imgproc,
    imgcodecs,
    types
};

use cast::{f64};

#[derive(Debug)]
pub struct Skew<'s> {
  src: &'s str,
  dst: &'s str,
  image: cv::Mat,
  threshold: f64,
  boundingbox: cv::RotatedRect,
  padding_top: i32,
  padding_left: i32
}

impl<'s> Skew<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> Skew<'s> {
    Skew {
      src: src,
      dst: dst,
      image: cv::Mat::default().unwrap(),
      threshold: 0.0,
      boundingbox: cv::RotatedRect::default().unwrap(),
      padding_top: 0,
      padding_left: 0
    }
  }

  pub fn grayscale(mut self) -> Skew<'s> {
    let image = imgcodecs::imread(self.src.as_ref(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
    self.image = image;
    self
  }

  pub fn invert(mut self) -> Skew<'s> {
    let mut image = cv::Mat::default().unwrap();
    let threshold = imgproc::threshold(&self.image, &mut image, 0.0, 255.0, imgproc::THRESH_BINARY_INV | imgproc::THRESH_OTSU).unwrap();

    println!("Inverted with optimal threshold: {:?}", threshold);

    self.image = image;
    self.threshold = threshold;
    self
  }

  pub fn unskew(mut self) -> Skew<'s> {
    let src = imgcodecs::imread(self.src.as_ref(), imgcodecs::IMREAD_UNCHANGED).unwrap();
    let mut image = cv::Mat::default().unwrap();

    self = self._bounding_box(); 

    let size = src.size().unwrap();
    let center = self._recalculate_center();
    let mut angle = self.boundingbox.angle().unwrap();
    let scalar = cv::Scalar_::new(0.0, 0.0, 0.0, 0.0);

    println!("Center: {:?} deg", center);

    // min_area_rect returns a value in the range [-90, 0). As the rectangle rotates 
    // cw the angle value goes towards zero, when zero is reached, angle is set back to -90.
    if angle < -45.0 {
        angle = 90.0 + angle;
    } 

    // Calculate an affinate matrix of 2D rotation
    let matrix = imgproc::get_rotation_matrix_2d(center, angle as f64, 1.0).unwrap();

    //Apply affine transformation
    let _result = imgproc::warp_affine(&src, &mut image, &matrix, size, imgproc::INTER_LINEAR, cv::BORDER_REPLICATE, scalar);

    println!("Rotated: {:?} deg", angle);

    self.image = image;
    self
  }

  pub fn pad(mut self) -> Skew<'s> {
    let mut image = cv::Mat::default().unwrap();

    let rows = self.image.rows().unwrap();
    let cols = self.image.cols().unwrap();

    let top = (0.2 * rows as f32) as i32;
    let left = (0.2 * cols as f32) as i32;
    let right = left;
    let bottom = top;

    println!("top: {:?}", top);

    let scalar = cv::Scalar_::new(0.0, 0.0, 0.0, 0.0);
    let _result = cv::copy_make_border(&self.image, &mut image, top, bottom, left, right, cv::BORDER_CONSTANT, scalar);

    self.image = image;
    self.padding_top = top;
    self.padding_left = left;
    self
  }

  pub fn save(self) -> Result<&'static str, opencv::Error> {
    let params = types::VectorOfint::new();

    let result = imgcodecs::imwrite(self.dst.as_ref(), &self.image, &params);

    match result {
        Ok(_) => return Ok("File was written"),
        Err(e) => return Err(e),
    };
  }

  /**
   * Sets a bounding box
   */
  fn _bounding_box(mut self) -> Skew<'s> {
    let mut points: types::VectorOfPoint = types::VectorOfPoint::new();

    let cols = self.image.cols().unwrap();
    let rows = self.image.rows().unwrap();

    for col in 0..cols {
        for row in 0..rows {
            let pixel = self.image.at_2d::<u8>(row, col).unwrap();

            // If pixel value is higher than determined threshold value, push coordinates to point vector
            if f64(*pixel) > self.threshold {
                let point  = cv::Point_::new(col, row);
                points.push(point);
            }
        }
    }

    let boundingbox = imgproc::min_area_rect(&points).unwrap();
    self.boundingbox = boundingbox;
    self
  }

  /**
   * Get recalculated center after padding
   */
  fn _recalculate_center(&self) -> cv::Point2f {
    let center = &self.boundingbox.center().unwrap();

    let row = self.padding_top as f32;
    let col = self.padding_left as f32;
    let point  = cv::Point_::new(center.x - col, center.y - row);
    point
  }
}
