use magnus::{function, prelude::*, Error, Ruby, RArray};
extern crate autopilot;

fn key_flags_from_symbols(symbols: Vec<String>) -> Vec<autopilot::key::Flag> {
    symbols
        .iter()
        .filter_map(|symbol| key_flag_from_symbol(symbol))
        .collect::<Vec<autopilot::key::Flag>>()
}

fn key_flag_from_symbol(symbol: &String) -> Option<autopilot::key::Flag> {
    match symbol.as_str() {
        "shift" => Some(autopilot::key::Flag::Shift),
        "control" => Some(autopilot::key::Flag::Control),
        "alt" => Some(autopilot::key::Flag::Alt),
        "meta" => Some(autopilot::key::Flag::Meta),
        "help" => Some(autopilot::key::Flag::Help),
        _ => None,
    }
}

fn type_string(string: String, _flag: RArray, wpm: f64, noise: f64) -> () {
    let _flags = _flag.to_vec::<String>().unwrap();
    let flags = key_flags_from_symbols(_flags);
    autopilot::key::type_string(&string, &flags, wpm, noise);
}

fn toggle(_key: char, down: bool, _flags: RArray, modifier_delay_ms: u64) -> () {
    let flags = key_flags_from_symbols(_flags.to_vec::<String>().unwrap());
    let key = autopilot::key::Character(_key);
    autopilot::key::toggle(&key, down, &flags, modifier_delay_ms);
}

fn tap(_key: char, _flags: RArray, delay_ms: u64, modifier_delay_ms: u64) -> () {
    let flags = key_flags_from_symbols(_flags.to_vec::<String>().unwrap());
    let key = autopilot::key::Character(_key);
    autopilot::key::tap(&key, &flags, delay_ms, modifier_delay_ms);
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Deskbot")?;
    module.define_singleton_method("_type_string", function!(type_string, 4))?;
    module.define_singleton_method("_toggle", function!(toggle, 4))?;
    module.define_singleton_method("_tap", function!(tap, 4))?;
    Ok(())
}
