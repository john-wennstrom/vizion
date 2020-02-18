extern crate clap;
extern crate unskew;
extern crate convolution;
extern crate face_detection;

use unskew::Unskew;
use convolution::Convolve;
use face_detection::FaceDetection;

use clap::{
  crate_version, 
  crate_authors, 
  Arg, 
  App, 
  SubCommand, 
  AppSettings
};

fn main() {
  let matches = App::new("Vizion")
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::ColoredHelp)
    .version(crate_version!())
    .author(crate_authors!())
    .about("Computer vision library")

    .subcommand(SubCommand::with_name("unskew")
      .about("Unskews an image with text")
      .version("0.1.0")
      .author(crate_authors!())
      .arg(Arg::with_name("SRC")
        .help("Source file")
        .required(true)
        .index(1))
      .arg(Arg::with_name("DST")
          .help("Destination file")
          .required(true)
          .index(2)))
    
    .subcommand(SubCommand::with_name("face-detection")
      .about("Facedetector")
      .version("0.1.0")
      .author(crate_authors!())
      .arg(Arg::with_name("SRC")
        .help("Input image")
        .required(true)
        .index(1))
      .arg(Arg::with_name("DST")
        .help("Destination file")
        .required(true)
        .index(2)))

    .subcommand(SubCommand::with_name("convolution")
        .about("Convolution")
        .version("0.1.0")
        .author(crate_authors!())
        .arg(Arg::with_name("SRC")
          .help("Input image")
          .required(true)
          .index(1))
        .arg(Arg::with_name("DST")
          .help("Destination file")
          .required(true)
          .index(2)))
    .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("unskew") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();

      let image = Unskew::new(&src, &dst)
      .grayscale()
      .invert()
      .pad()
      .unskew()
      .save();

      println!("Result: {:?}", image);
    }

    if let Some(ref matches) = matches.subcommand_matches("face-detection") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();
     
      let face = FaceDetection::new(&src, &dst)
        .detect()
        .draw()
        .save();

        println!("Result: {:?}", face);
    }

    if let Some(ref matches) = matches.subcommand_matches("convolution") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();
     
      let convolved = Convolve::new(&src, &dst).convolve();

      println!("Result: {:?}", convolved);
    }
}
