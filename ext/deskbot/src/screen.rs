extern crate autopilot;
use std::collections::HashMap;

pub fn get_color(x: f64, y: f64) -> Option<HashMap<String, u8>> {
    let color = autopilot::screen::get_color(autopilot::geometry::Point::new(x, y));
    match color {
        Ok(color) => Some(HashMap::from([
            ("r".to_string(), color.0[0]),
            ("g".to_string(), color.0[1]),
            ("b".to_string(), color.0[2]),
            ("a".to_string(), color.0[3])
        ])),
        Err(_) => None
    }
}
