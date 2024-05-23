use magnus::{function, prelude::*, Error, Ruby};
extern crate autopilot;

mod keys;
mod mouse;
mod screen;

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

    module.define_singleton_method("_get_color", function!(screen::get_color, 2))?;
    module.define_singleton_method("_screen_size", function!(screen::size, 0))?;
    module.define_singleton_method("_screen_scale", function!(screen::scale, 0))?;
    module.define_singleton_method("_is_point_visible", function!(screen::is_point_visible, 2))?;
    module.define_singleton_method("_is_rect_visible", function!(screen::is_rect_visible, 4))?;
    Ok(())
}
