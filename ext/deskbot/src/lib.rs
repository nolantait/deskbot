use magnus::{class, function, method, prelude::*, Error, Ruby};
extern crate autopilot;

mod bitmap;
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

    module.define_singleton_method(
        "_capture_screen_portion",
        function!(bitmap::capture_screen_portion, 4),
    )?;
    module.define_singleton_method("_capture_screen", function!(bitmap::capture_screen, 0))?;

    let bitmap = module.define_class("Bitmap", class::object())?;
    bitmap.define_method("_bounds", method!(bitmap::Bitmap::bounds, 0))?;
    bitmap.define_method("_find", method!(bitmap::Bitmap::find, 2))?;
    bitmap.define_method("_all", method!(bitmap::Bitmap::all, 2))?;
    bitmap.define_method("_count", method!(bitmap::Bitmap::count, 2))?;
    bitmap.define_method("_find_color", method!(bitmap::Bitmap::find_color, 2))?;
    bitmap.define_method("_all_color", method!(bitmap::Bitmap::all_color, 2))?;
    Ok(())
}
