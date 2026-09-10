extern crate autopilot;

use image::{open, GenericImageView};
use magnus::{Error, Ruby};
use std::collections::HashMap;
use std::path::PathBuf;

fn last_screenshot_path() -> PathBuf {
    std::env::temp_dir().join("deskbot-last-screenshot.png")
}

extern crate opencv;

use opencv::{
    core::{self, Mat, MatTraitConst, Rect, Scalar},
    imgproc, Result,
};

#[magnus::wrap(class = "Deskbot::Providers::Autopilot::Bitmap")]
pub struct Bitmap(autopilot::bitmap::Bitmap);

impl Bitmap {
    const MAX_COLOR_DELTA: f64 = 441.672_955_930_1; // => (3.0f64 * 255.0f64 * 255.0f64).sqrt();

    fn new(bitmap: autopilot::bitmap::Bitmap) -> Self {
        Bitmap(bitmap)
    }

    pub fn save(&self, path: String) -> Result<bool, magnus::Error> {
        match self.0.image.save(path) {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::new(
                Ruby::get().unwrap().exception_runtime_error(),
                "Could not save the image",
            )),
        }
    }

    pub fn bounds(&self) -> HashMap<String, f64> {
        let bounds = self.0.bounds();

        HashMap::from([
            ("x".to_string(), bounds.origin.x),
            ("y".to_string(), bounds.origin.y),
            ("width".to_string(), bounds.size.width),
            ("height".to_string(), bounds.size.height),
        ])
    }

    pub fn get_pixel(&self, x: f64, y: f64) -> Vec<u8> {
        let point = autopilot::geometry::Point::new(x, y);
        let value = self.0.get_pixel(point).0;
        // For some reason I have to convert to a Vec<u8>. Magnus doesn't like [u8; 4]
        return value.to_vec();
    }

    pub fn find_color(
        &self,
        color: [u8; 4],
        tolerance: Option<f64>,
    ) -> Option<HashMap<String, f64>> {
        if let Some(found) = self.0.find_color(image::Rgba(color), tolerance, None, None) {
            return Some(HashMap::from([
                ("x".to_string(), found.x),
                ("y".to_string(), found.y),
            ]));
        }
        None
    }

    pub fn find(&self, image_path: String, tolerance: Option<f64>) -> Option<HashMap<String, f64>> {
        let src = match opencv::imgcodecs::imread(
            last_screenshot_path(),
            opencv::imgcodecs::IMREAD_COLOR,
        ) {
            Ok(src) => src,
            Err(error) => panic!("Could not read the image: {}", error),
        };
        let templ = match opencv::imgcodecs::imread(&image_path, opencv::imgcodecs::IMREAD_COLOR) {
            Ok(templ) => templ,
            Err(error) => panic!("Could not read the image: {}", error),
        };

        // Create Mat for the result
        let mut dst = Mat::default();
        let mask = Mat::default();

        // Perform template matching
        match imgproc::match_template(
            &src,
            &templ,
            &mut dst,
            imgproc::TemplateMatchModes::TM_SQDIFF_NORMED.into(),
            &mask,
        ) {
            Ok(_) => {}
            Err(error) => panic!("Could not match the template: {}", error),
        }

        // Find the location of the best match
        let (mut min_val, mut max_val) = (0.0, 0.0);
        let mut min_point = core::Point { x: 0, y: 0 };
        let mut max_point = core::Point { x: 0, y: 0 };

        match core::min_max_loc(
            &dst,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_point),
            Some(&mut max_point),
            &mask,
        ) {
            Ok(_) => {}
            Err(error) => panic!("Could not find the min max loc: {}", error),
        }

        if min_val > tolerance.unwrap_or(0.0) {
            return None;
        }

        Some(HashMap::from([
            ("x".to_string(), min_point.x as f64),
            ("y".to_string(), min_point.y as f64),
        ]))
    }

    pub fn all_color(&self, color: [u8; 4], tolerance: Option<f64>) -> Vec<HashMap<String, f64>> {
        // autopilot's find_every_color iterates scaled pixel coordinates and
        // reads out of bounds on scaled displays (see screen.rs for a similar
        // autopilot bug). Scan the captured pixels directly instead.
        //
        // The image is captured at physical resolution, so the raw scan
        // indices are pixels. Convert them to the bitmap's point (logical)
        // coordinate space by dividing by the bitmap's scale, so a
        // found color can be fed straight to move_mouse (after offsetting by
        // the capture-region origin). This keeps all_color consistent with
        // find_color / find, which autopilot already returns as points.
        let scale = self.0.scale;
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let mut results = vec![];
        for y in 0..self.0.image.height() {
            for x in 0..self.0.image.width() {
                if Self::color_matches(color, self.0.image.get_pixel(x, y), tolerance) {
                    results.push(HashMap::from([
                        ("x".to_string(), x as f64 / scale),
                        ("y".to_string(), y as f64 / scale),
                    ]));
                }
            }
        }
        results
    }

    // Mirrors autopilot's colors_match: an exact rgba match at a nil/zero
    // tolerance, otherwise an euclidean rgb distance check bounded by the
    // tolerance.
    fn color_matches(color: [u8; 4], pixel: image::Rgba<u8>, tolerance: Option<f64>) -> bool {
        let tolerance = tolerance.unwrap_or(0.0);
        let pixel = pixel.0;
        if tolerance == 0.0 {
            return color == pixel;
        }

        let dr = f64::from(color[0]) - f64::from(pixel[0]);
        let dg = f64::from(color[1]) - f64::from(pixel[1]);
        let db = f64::from(color[2]) - f64::from(pixel[2]);
        (dr * dr + dg * dg + db * db).sqrt() <= tolerance * Self::MAX_COLOR_DELTA
    }

    pub fn all(&self, image_path: String, tolerance: Option<f64>) -> Vec<HashMap<String, f64>> {
        let mut results = vec![];

        let mut image = self.load_image(last_screenshot_path());
        let template_image = self.load_image(&image_path);
        let mut matches: Vec<core::Point> = Vec::new();

        // Fake out min_val for first run through loop
        loop {
            match self.match_template_and_replace(
                &mut image,
                &template_image,
                tolerance.unwrap_or(0.5),
            ) {
                Some(point) => {
                    matches.push(point);
                }
                None => break,
            }
        }

        matches.iter().for_each(|point| {
            results.push(HashMap::from([
                ("x".to_string(), point.x as f64),
                ("y".to_string(), point.y as f64),
            ]));
        });

        results
    }

    pub fn count(&self, image_path: String, tolerance: Option<f64>) -> u64 {
        if let Ok(image) = open(image_path) {
            let needle = autopilot::bitmap::Bitmap::new(image, None);
            return self.0.count_of_bitmap(&needle, tolerance, None, None);
        }
        0
    }

    fn minimum_point(&self, mat: &Mat) -> (core::Point, f64) {
        let mut min_val = 1.;
        let mut min_point = core::Point { x: 0, y: 0 };

        match core::min_max_loc(
            &mat,
            Some(&mut min_val),
            None,
            Some(&mut min_point),
            None,
            &core::no_array(),
        ) {
            Ok(_) => {}
            Err(error) => panic!("Could not find the min max loc: {}", error),
        }

        (min_point, min_val)
    }

    fn load_image(&self, path: impl AsRef<std::ffi::OsStr>) -> Mat {
        match opencv::imgcodecs::imread(path, opencv::imgcodecs::IMREAD_COLOR) {
            Ok(src) => src,
            Err(error) => panic!("Could not read the image: {}", error),
        }
    }

    // fn write_image(&self, path: &str, image: &Mat) {
    //     match opencv::imgcodecs::imwrite(path, image, &core::Vector::<i32>::new()) {
    //         Ok(_) => {}
    //         Err(error) => panic!("Could not write the image: {}", error),
    //     }
    // }

    fn match_template(&self, image: &Mat, template: &Mat, threshold: f64) -> Option<core::Point> {
        let mut result = Mat::default();

        imgproc::match_template_def(image, &template, &mut result, imgproc::TM_SQDIFF_NORMED)
            .unwrap();

        let (min_point, min_val) = self.minimum_point(&result);

        if min_val > threshold {
            return None;
        }

        Some(min_point)
    }

    fn match_template_and_replace(
        &self,
        image: &mut Mat,
        template: &Mat,
        threshold: f64,
    ) -> Option<core::Point> {
        let min_point = self.match_template(image, template, threshold)?;
        let template_size = template.size().unwrap();

        let rect = Rect {
            x: min_point.x,
            y: min_point.y,
            width: template_size.width as i32 + 1,
            height: template_size.height as i32 + 1,
        };

        imgproc::rectangle(image, rect, Scalar::new(0.0, 0.0, 0.0, 0.0), -1, 8, 0).unwrap();

        Some(min_point)
    }
}

pub fn capture_screen_portion(x: f64, y: f64, width: f64, height: f64) -> Option<Bitmap> {
    let point = autopilot::geometry::Point::new(x, y);
    let size = autopilot::geometry::Size::new(width, height);
    let rect = autopilot::geometry::Rect::new(point, size);
    let image = autopilot::bitmap::capture_screen_portion(rect);

    match image {
        Ok(image) => {
            image.image.save(&last_screenshot_path()).unwrap();
            Some(Bitmap::new(image))
        }
        Err(_) => None,
    }
}

pub fn capture_screen() -> Option<Bitmap> {
    let image = autopilot::bitmap::capture_screen();

    match image {
        Ok(image) => {
            image.image.save(&last_screenshot_path()).unwrap();
            Some(Bitmap::new(image))
        }
        Err(_) => None,
    }
}
