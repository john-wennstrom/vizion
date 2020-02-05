extern crate clap;
extern crate unskew;
extern crate face;

use unskew::Unskew;
use face::Face;

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
    
    .subcommand(SubCommand::with_name("face")
      .about("Facedetector")
      .version("0.1.0")
      .author(crate_authors!())
      .arg(Arg::with_name("detector")
        .help("Detector path")
        .required(true)
        .index(1))
      .arg(Arg::with_name("dataset")
        .help("Dataset path")
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

      println!("1: {:?}", image);
    }

    if let Some(ref matches) = matches.subcommand_matches("face") {
      let detector_path = matches.value_of("detector").unwrap();
      let dataset_path = matches.value_of("dataset").unwrap();

      let _face = Face::new(dataset_path, "", &detector_path, "")
      .run()
      .detect();
    }
}
