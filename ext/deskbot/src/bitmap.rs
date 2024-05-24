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

    pub fn find(&self, image_path: String) -> Option<HashMap<String, f64>> {
        if let Ok(image) = open(image_path) {
            let bitmap = autopilot::bitmap::Bitmap::new(image, None);
            let bounds = bitmap.bounds();

            let hash = HashMap::from([
                ("x".to_string(), bounds.origin.x),
                ("y".to_string(), bounds.origin.y),
                ("width".to_string(), bounds.size.width),
                ("height".to_string(), bounds.size.height),
            ]);

            return Some(hash)
        }
        None
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
