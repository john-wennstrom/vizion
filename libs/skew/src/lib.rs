extern crate imageproc;

use std::path::Path;
use imageproc::edges::canny;
use imageproc::hough::{detect_lines, PolarLine, LineDetectionOptions};
use image::{GrayImage};
use std::f32;
use std::default::Default;

struct Settings {
    max_threshold: f32
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            max_threshold: 5.0_f32.sqrt() * 2.0 * 255.0
        }
    }
}

pub fn determine_skew(image: &GrayImage, output_dir: &Path) -> Vec<PolarLine> {
    return _determine_skew(&image, &output_dir);
}

fn _determine_skew(image: &GrayImage, output_dir: &Path) -> Vec<PolarLine> {
    let settings = Settings{..Default::default()};

    // Detect edges using Canny algorithm
    let edges = canny(image, settings.max_threshold * 0.01, settings.max_threshold * 0.2);

    let canny_path = output_dir.join("canny.png");
    edges.save(&canny_path).unwrap();

    // Detect lines using Hough transform
    let options = LineDetectionOptions {
        vote_threshold: 10,
        suppression_radius: 1
    };

    let lines: Vec<PolarLine> = detect_lines(&edges, options);

    return lines;
}
