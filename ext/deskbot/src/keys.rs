use magnus::RArray;
extern crate autopilot;

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
