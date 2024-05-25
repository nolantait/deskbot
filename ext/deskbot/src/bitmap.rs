extern crate autopilot;

use image::open;
use std::collections::HashMap;

#[magnus::wrap(class = "Deskbot::Bitmap")]
pub struct Bitmap(autopilot::bitmap::Bitmap);

impl Bitmap {
    fn new(bitmap: autopilot::bitmap::Bitmap) -> Self {
        Bitmap(bitmap)
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
        if let Ok(image) = open(image_path) {
            let needle = autopilot::bitmap::Bitmap::new(image, None);

            if let Some(found) = self.0.find_bitmap(&needle, tolerance, None, None) {
                return Some(HashMap::from([
                    ("x".to_string(), found.x),
                    ("y".to_string(), found.y),
                ]));
            }
        }
        None
    }

    pub fn all_color(&self, color: [u8; 4], tolerance: Option<f64>) -> Vec<HashMap<String, f64>> {
        let mut results = vec![];
        for found in self
            .0
            .find_every_color(image::Rgba(color), tolerance, None, None)
        {
            results.push(HashMap::from([
                ("x".to_string(), found.x),
                ("y".to_string(), found.y),
            ]));
        }
        results
    }

    pub fn all(&self, image_path: String, tolerance: Option<f64>) -> Vec<HashMap<String, f64>> {
        let mut results = vec![];
        if let Ok(image) = open(image_path) {
            let needle = autopilot::bitmap::Bitmap::new(image, None);
            for found in self.0.find_every_bitmap(&needle, tolerance, None, None) {
                results.push(HashMap::from([
                    ("x".to_string(), found.x),
                    ("y".to_string(), found.y),
                ]));
            }
        }
        results
    }

    pub fn count(&self, image_path: String, tolerance: Option<f64>) -> u64 {
        if let Ok(image) = open(image_path) {
            let needle = autopilot::bitmap::Bitmap::new(image, None);
            return self.0.count_of_bitmap(&needle, tolerance, None, None);
        }
        0
    }
}

pub fn capture_screen_portion(x: f64, y: f64, width: f64, height: f64) -> Option<Bitmap> {
    let point = autopilot::geometry::Point::new(x, y);
    let size = autopilot::geometry::Size::new(width, height);
    let rect = autopilot::geometry::Rect::new(point, size);
    let image = autopilot::bitmap::capture_screen_portion(rect);

    match image {
        Ok(image) => Some(Bitmap::new(image)),
        Err(_) => None,
    }
}

pub fn capture_screen() -> Option<Bitmap> {
    let image = autopilot::bitmap::capture_screen();

    match image {
        Ok(image) => Some(Bitmap::new(image)),
        Err(_) => None,
    }
}
