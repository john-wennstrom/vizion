extern crate opencv;
extern crate cast;

use opencv::{
  core as cv,
  imgcodecs,
  imgproc,
  objdetect::{CascadeClassifier},
  types,
  prelude::*
};

pub struct FaceDetection<'s> {
    dst_path: &'s str,
    cascade_classifier: CascadeClassifier,
    gray_image: cv::Mat,
    final_image: cv::Mat,
    detections: types::VectorOfRect
}

impl<'s> FaceDetection<'s> {
  pub fn new(src: &'s str, dst: &'s str) -> FaceDetection<'s> {
    FaceDetection {
      dst_path: dst,
      cascade_classifier: CascadeClassifier::new("examples/classifiers/haarcascade_frontalface_default.xml").unwrap(),
      gray_image: FaceDetection::_get_gray_image(&src),
      final_image: FaceDetection::_get_original_image(&src),
      detections: types::VectorOfRect::new()
    }
  }

  pub fn detect(mut self) -> FaceDetection<'s> {
    let mut objects = types::VectorOfRect::new();

    let _faces = self.cascade_classifier.detect_multi_scale(
      &self.gray_image,
      &mut objects,
      1.1,
      10,
      0,
      cv::Size_::new(6, 6),
      cv::Size_::new(1000, 1000)
    );

    self.detections = objects;
    self
  }

  pub fn draw(mut self) -> FaceDetection<'s> {

    let color = cv::Scalar_::new(200.0, 155.0, 122.0, 0.0);

    for rect in &self.detections {
      println!("{:?}", rect);

      let _draw_result = imgproc::rectangle(
        &mut self.final_image,
        rect,
        color,
        2,
        imgproc::LINE_8,
        0
      );
    }

    self
  }

  pub fn save(self) -> Result<&'static str, opencv::Error> {
    let params = types::VectorOfint::new();

    let result = imgcodecs::imwrite(&self.dst_path, &self.final_image, &params);

    match result {
        Ok(_) => return Ok("File was written"),
        Err(e) => return Err(e),
    };
  }

  fn _get_gray_image(path: &str) -> cv::Mat {
    let image = imgcodecs::imread(path.as_ref(), imgcodecs::IMREAD_GRAYSCALE).unwrap();

    image
  }

  fn _get_original_image(path: &str) -> cv::Mat {
    let image = imgcodecs::imread(path.as_ref(), imgcodecs::IMREAD_COLOR).unwrap();

    image
  }
}
