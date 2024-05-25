extern crate autopilot;
use std::collections::HashMap;

pub fn get_color(x: f64, y: f64) -> Option<Vec<u8>> {
    let color = autopilot::screen::get_color(autopilot::geometry::Point::new(x, y));
    match color {
        Ok(color) => Some(color.0.to_vec()),
        Err(_) => None
    }
}

pub fn size() -> HashMap<String, f64> {
    let size = autopilot::screen::size();

    HashMap::from([
        ("width".to_string(), size.width),
        ("height".to_string(), size.height)
    ])
}

pub fn scale() -> f64 {
    autopilot::screen::scale()
}

pub fn is_point_visible(x: f64, y: f64) -> bool {
    let point = autopilot::geometry::Point::new(x, y);
    autopilot::screen::is_point_visible(point)
}

pub fn is_rect_visible(x: f64, y: f64, width: f64, height: f64) -> bool {
    let point = autopilot::geometry::Point::new(x, y);
    let size = autopilot::geometry::Size::new(width, height);
    autopilot::screen::is_rect_visible(autopilot::geometry::Rect::new(point, size))
}
