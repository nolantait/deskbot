extern crate autopilot;

pub fn alert(message: String) -> () {
    autopilot::alert::alert(&message.as_str(), None, None, None);
}
