use magnus::{class, function, method, prelude::*, Error, Ruby};
extern crate autopilot;

mod bitmap;
mod keys;
mod mouse;
mod screen;
mod alert;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let main_module = ruby.define_module("Deskbot")?.define_module("Providers")?;
    let module = main_module.define_module("Autopilot")?;
    module.define_singleton_method("type", function!(keys::type_string, 4))?;
    module.define_singleton_method("toggle_key", function!(keys::toggle_key, 4))?;
    module.define_singleton_method("tap_key", function!(keys::tap_key, 4))?;

    module.define_singleton_method("mouse_location", function!(mouse::location, 0))?;
    module.define_singleton_method("move_mouse", function!(mouse::move_to, 2))?;
    module.define_singleton_method("toggle_mouse", function!(mouse::toggle, 2))?;
    module.define_singleton_method("click", function!(mouse::click, 2))?;
    module.define_singleton_method("scroll", function!(mouse::scroll, 2))?;
    module.define_singleton_method("smooth_move_mouse", function!(mouse::smooth_move, 3))?;

    module.define_singleton_method("color_at", function!(screen::get_color, 2))?;
    module.define_singleton_method("screen_size", function!(screen::size, 0))?;
    module.define_singleton_method("screen_scale", function!(screen::scale, 0))?;
    module.define_singleton_method("is_point_visible", function!(screen::is_point_visible, 2))?;
    module.define_singleton_method("is_area_visible", function!(screen::is_rect_visible, 4))?;

    module.define_singleton_method("alert", function!(alert::alert, 1))?;

    module.define_singleton_method(
        "capture_screen_area",
        function!(bitmap::capture_screen_portion, 4),
    )?;
    module.define_singleton_method("capture_screen", function!(bitmap::capture_screen, 0))?;

    let bitmap = module.define_class("Bitmap", class::object())?;
    bitmap.define_method("bounds", method!(bitmap::Bitmap::bounds, 0))?;
    bitmap.define_method("find", method!(bitmap::Bitmap::find, 2))?;
    bitmap.define_method("all", method!(bitmap::Bitmap::all, 2))?;
    bitmap.define_method("count", method!(bitmap::Bitmap::count, 2))?;
    bitmap.define_method("find_color", method!(bitmap::Bitmap::find_color, 2))?;
    bitmap.define_method("all_color", method!(bitmap::Bitmap::all_color, 2))?;
    bitmap.define_method("color_at", method!(bitmap::Bitmap::get_pixel, 2))?;
    bitmap.define_method("save", method!(bitmap::Bitmap::save, 1))?;
    Ok(())
}
