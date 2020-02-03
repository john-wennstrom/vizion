extern crate opencv;
extern crate cast;

use opencv::{
    core,
    prelude::*,
    imgproc,
    imgcodecs,
    types
};

use cast::f64;

pub fn unskew(src: &str, dst: &str) -> opencv::Result<()> {
    let img_gray = imgcodecs::imread(src.as_ref(), imgcodecs::IMREAD_GRAYSCALE).unwrap();
    
    let img_unskew = _bounding_box_unskew(img_gray);

    let params = types::VectorOfint::new();

    let img = match img_unskew {
        Ok(img) => img,
        Err(e) => return Err(e),
    };

    let result = imgcodecs::imwrite(dst.as_ref(), &img, &params);

    match result {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}


fn _bounding_box_unskew(img_gray: opencv::core::Mat) -> opencv::Result<opencv::core::Mat> {
    let mut img_inverted = core::Mat::default()?;
    let mut img_rotated = core::Mat::default()?;

    // Invert the text and determine the optimal threshold value using Otsu's
    let threshold_result = imgproc::threshold(
        &img_gray, 
        &mut img_inverted, 
        0.0, 
        255.0, 
        imgproc::THRESH_BINARY_INV | imgproc::THRESH_OTSU
    );

    // Unwrap the threshold result or return error
    let threshold = match threshold_result {
        Ok(threshold) => threshold,
        Err(e) => return Err(e),
    };

    // Instantiate new vector of points
    let mut points: types::VectorOfPoint = types::VectorOfPoint::new();

    // Iterate pixels in image
    /**
        TODO: Make filter if possible
      
        fn hof() {
            let filter = |predicate: fn(&i32) -> bool, xs: Vec<i32>| {
                // A good Reddit post on how Filter works https://www.reddit.com/r/rust/comments/3bmua6/can_someone_help_me_understand_stditerfilter/
                xs.into_iter().filter(predicate).collect::<Vec<i32>>()
            };

            let is_even = |x: &i32| x % 2 == 0;

            let result = filter(is_even, vec![1, 2, 3, 4, 5, 6]);

            assert_eq!(result, vec![2, 4, 6]);
        }
    */
    for col in 0..img_inverted.cols().unwrap() {
        for row in 0..img_inverted.rows().unwrap() {
            let pixel = img_inverted.at_2d::<u8>(row, col).unwrap();

            // If pixel value is higher than determined threshold value, push coordinates to point vector
            if f64(*pixel) > threshold {
                let point  = core::Point_::new(col, row);
                points.push(point);
            }
        }
    }

    let rectangle_result = imgproc::min_area_rect(&points);

    let rectangle = match rectangle_result {
        Ok(rectangle) => rectangle,
        Err(e) => return Err(e),
    };

    let mut angle = rectangle.angle().unwrap();

    // min_area_rect returns a value in the range [-90, 0). As the rectangle rotates 
    // cw the angle value goes towards zero, when zero is reached, angle is set back to -90.
    if angle < -45.0 {
        angle = 90.0 + angle;
    } 

    println!("Rotated: {:?} deg", angle);

    let center = rectangle.center().unwrap();
    let size = img_gray.size().unwrap();

    // Calculate an affinate matrix of 2D rotation
    let m = imgproc::get_rotation_matrix_2d(center, angle as f64, 1.0);

    let scalar = core::Scalar_::new(0.0, 0.0, 0.0, 0.0);

    //Apply affine transformation
    let warp_result = imgproc::warp_affine(
        &img_gray, 
        &mut img_rotated,
        &m.unwrap(),
        size,
        imgproc::INTER_LINEAR,
        core::BORDER_REPLICATE,
        scalar
    );

    match warp_result {
        Ok(_) => return Ok(img_rotated),
        Err(e) => return Err(e),
    };
}
