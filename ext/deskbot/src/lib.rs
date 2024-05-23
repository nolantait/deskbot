use magnus::{function, prelude::*, Error, RArray, Ruby};
extern crate autopilot;

mod keys {
    use super::*;

    fn key_flags(symbols: Vec<String>) -> Vec<autopilot::key::Flag> {
        symbols
            .iter()
            .filter_map(|symbol| key_flag(symbol))
            .collect::<Vec<autopilot::key::Flag>>()
    }

    fn key_flag(symbol: &String) -> Option<autopilot::key::Flag> {
        match symbol.as_str() {
            "shift" => Some(autopilot::key::Flag::Shift),
            "control" => Some(autopilot::key::Flag::Control),
            "alt" => Some(autopilot::key::Flag::Alt),
            "meta" => Some(autopilot::key::Flag::Meta),
            "help" => Some(autopilot::key::Flag::Help),
            _ => None,
        }
    }

    pub fn type_string(string: String, _flag: RArray, wpm: f64, noise: f64) -> () {
        let _flags = _flag.to_vec::<String>().unwrap();
        let flags = key_flags(_flags);
        autopilot::key::type_string(&string, &flags, wpm, noise);
    }

    pub fn toggle_key(_key: char, down: bool, _flags: RArray, modifier_delay_ms: u64) -> () {
        let flags = key_flags(_flags.to_vec::<String>().unwrap());
        let key = autopilot::key::Character(_key);
        autopilot::key::toggle(&key, down, &flags, modifier_delay_ms);
    }

    pub fn tap_key(_key: char, _flags: RArray, delay_ms: u64, modifier_delay_ms: u64) -> () {
        let flags = key_flags(_flags.to_vec::<String>().unwrap());
        let key = autopilot::key::Character(_key);
        autopilot::key::tap(&key, &flags, delay_ms, modifier_delay_ms);
    }
}

mod mouse {
    use super::*;
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
        return HashMap::from([
            ("x".to_string(), point.x),
            ("y".to_string(), point.y),
        ]);

    }

    pub fn move_to(x: f64, y: f64) -> bool {
        let command = autopilot::mouse::move_to(autopilot::geometry::Point::new(x, y));
        match command {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn smooth_move(x: f64, y: f64, duration: Option<f64>) -> bool {
        let command = autopilot::mouse::smooth_move(
            autopilot::geometry::Point::new(x, y),
            duration
        );

        match command {
            Ok(_) => true,
            Err(_) => false
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
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Deskbot")?;
    module.define_singleton_method("_type_string", function!(keys::type_string, 4))?;
    module.define_singleton_method("_toggle_key", function!(keys::toggle_key, 4))?;
    module.define_singleton_method("_tap_key", function!(keys::tap_key, 4))?;

    module.define_singleton_method("_mouse_location", function!(mouse::location, 0))?;
    module.define_singleton_method("_move_mouse", function!(mouse::move_to, 2))?;
    module.define_singleton_method("_toggle_mouse", function!(mouse::toggle, 2))?;
    module.define_singleton_method("_click", function!(mouse::click, 2))?;
    module.define_singleton_method("_scroll", function!(mouse::scroll, 2))?;
    module.define_singleton_method("_smooth_move_mouse", function!(mouse::smooth_move, 3))?;
    Ok(())
}
