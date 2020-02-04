extern crate opencv;
extern crate cast;

use opencv::{
    core as cv,
    dnn,
    prelude::*,
    imgproc,
    imgcodecs,
    types
};

use cast::{f64};

pub struct Face<'s> {
  dataset_path: &'s str,
  embeddings_path: &'s str,
  detector_path: &'s str,
  embedding_model_path: &'s str,
  confidence: f32
}

impl<'s> Face<'s> {
  pub fn new(
    dataset_path: &'s str,
    embeddings_path: &'s str,
    detector_path: &'s str,
    embedding_model_path: &'s str
  ) -> Face<'s> {
    Face {
      dataset_path: dataset_path,
      embeddings_path: embeddings_path,
      detector_path: detector_path,
      embedding_model_path: embedding_model_path,
      confidence: 0.5
    }
  }

  pub fn load(mut self) -> Face<'s> {
    let prototxt = [self.detector_path, "deploy.prototxt"].concat();
    let caffe_model = [self.detector_path, "res10_300x300_ssd_iter_140000.caffemodel"].concat();

    let detector: dnn::Net = dnn::read_net_from_caffe(&prototxt, &caffe_model).unwrap();

    println!("{:?}", 1);

    self
  }
}