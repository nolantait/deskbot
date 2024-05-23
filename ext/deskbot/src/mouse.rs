use magnus::{function, prelude::*, Error, RArray, Ruby};
extern crate autopilot;
use std::collections::HashMap;

fn button(symbol: String) -> autopilot::mouse::Button {
    match symbol.as_str() {
        "left" => autopilot::mouse::Button::Left,
        "middle" => autopilot::mouse::Button::Middle,
        "right" => autopilot::mouse::Button::Right,
        _ => panic!("Invalid button"),
    }
}

fn scroll_direction(symbol: String) -> autopilot::mouse::ScrollDirection {
    match symbol.as_str() {
        "up" => autopilot::mouse::ScrollDirection::Up,
        "down" => autopilot::mouse::ScrollDirection::Down,
        _ => panic!("Invalid scroll direction"),
    }
}

pub fn location() -> HashMap<String, f64> {
    let point = autopilot::mouse::location();
    return HashMap::from([("x".to_string(), point.x), ("y".to_string(), point.y)]);
}

pub fn move_to(x: f64, y: f64) -> bool {
    let command = autopilot::mouse::move_to(autopilot::geometry::Point::new(x, y));
    match command {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn smooth_move(x: f64, y: f64, duration: Option<f64>) -> bool {
    let command = autopilot::mouse::smooth_move(autopilot::geometry::Point::new(x, y), duration);

    match command {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn toggle(_button: String, down: bool) -> () {
    let button = button(_button);
    autopilot::mouse::toggle(button, down);
}

pub fn click(_button: String, delay_ms: Option<u64>) -> () {
    let button = button(_button);
    autopilot::mouse::click(button, delay_ms);
}

pub fn scroll(direction: String, clicks: u32) -> () {
    autopilot::mouse::scroll(scroll_direction(direction), clicks);
}
