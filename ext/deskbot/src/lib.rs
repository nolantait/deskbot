use magnus::{function, prelude::*, Error, Ruby, Symbol};
extern crate autopilot;

fn key_flag_from_symbol(symbol: Symbol) -> Option<autopilot::key::Flag> {
    let name = symbol.name().unwrap().to_string();

    match name.as_str() {
        "shift" => Some(autopilot::key::Flag::Shift),
        "control" => Some(autopilot::key::Flag::Control),
        "alt" => Some(autopilot::key::Flag::Alt),
        "meta" => Some(autopilot::key::Flag::Meta),
        "help" => Some(autopilot::key::Flag::Help),
        _ => None,
    }
}

fn type_string(string: String, _flag: Symbol, wpm: f64, noise: f64) -> () {
    if let Some(flag) = key_flag_from_symbol(_flag) {
        autopilot::key::type_string(&string, &[flag], wpm, noise);
    } else {
        autopilot::key::type_string(&string, &[], wpm, noise);
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Deskbot")?;
    module.define_singleton_method("_type_string", function!(type_string, 4))?;
    Ok(())
}
