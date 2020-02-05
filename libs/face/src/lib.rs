extern crate opencv;
extern crate cast;

use opencv::{
  core as cv,
  dnn, 
  imgcodecs,
  imgproc
};
use std::path::{Path, PathBuf};
use std::{io, fs};

pub struct Face<'s> {
  dataset_path: &'s str,
  embeddings_path: &'s str,
  detector_path: &'s str,
  embedding_model_path: &'s str,
  confidence: f32,
  dataset: Result<Vec<PathBuf>, io::Error>,
  detector: dnn::Net
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
      confidence: 0.5,
      dataset: Ok(Vec::default()),
      detector: dnn::Net::default().unwrap()
    }
  }

  pub fn run(mut self) -> Face<'s> {
    
    let _embedder = &self._get_embedder();

    // TODO: Remove unwrap
    self.detector = self._get_detector().unwrap();
    &self._load_dataset();
    self
  }

  pub fn detect(mut self) -> Result<&'s str, opencv::Error> {

    let dataset = self.dataset.unwrap();

    for image_path in dataset {
      let path = image_path.to_str().unwrap();
      let mut dst = cv::Mat::default().unwrap();
      let src = imgcodecs::imread(path, imgcodecs::IMREAD_UNCHANGED).unwrap();

      println!("Processing {:?}", path);

      // Resize image
      let resize_result = imgproc::resize(
        &src, 
        &mut dst, 
        cv::Size_::new(300, 300), 
        1.2, 
        1.2, 
        imgproc::INTER_LINEAR
      ); 

      let mean = cv::Scalar_::new(100.0, 55.0, 0.0, 0.0);

      // Construct blob from image
      let image_blob = dnn::blob_from_image(
        &mut dst, 
        1.0,
        cv::Size_::new(300, 300),
        mean,
        false,
        false,
        cv::CV_32F
      );

      self.detector.set_input(
        &image_blob,
        "name".to_owned(),
        1.0,
        mean
      );

      println!("Result {:?}", image_blob.unwrap());
    }
    
    Ok("Worked")
  }

  /**
   * Loads paths of images into vector
   */
  fn _load_dataset(&mut self) -> io::Result<()> {
    let dir = Path::new(&self.dataset_path);

    let dataset = fs::read_dir(dir)?
      .map(|res| res.map(|entry| entry.path()))
      .collect::<Result<Vec<_>, io::Error>>();

    println!("{:?}", dataset);

    self.dataset = dataset;

    Ok(())
  }

  /**
   * Provides our serialized face detector from disk
   */
  fn _get_detector(&self) -> Result<dnn::Net, opencv::Error> {
    let prototxt = Path::new(&self.detector_path).join("deploy.prototxt");
    let caffe_model = Path::new(&self.detector_path).join("res10_300x300_ssd_iter_140000.caffemodel");

    let detector_result = dnn::read_net_from_caffe(
      &prototxt.to_str().unwrap(), 
      &caffe_model.to_str().unwrap()
    );

    detector_result
  }

  /**
   * Provides our serialized face embedding model from disk
   */
  fn _get_embedder(&self) -> Result<dnn::Net, opencv::Error> {
    let embedding = Path::new(&self.detector_path).join("deploy.prototxt");

    let embedding_result = dnn::read_net_from_torch(
      &embedding.to_str().unwrap(),
      true,
      true
    );

    embedding_result
  }
}
