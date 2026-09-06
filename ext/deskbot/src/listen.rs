use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex, OnceLock,
};
use std::time::Duration;

use device_query::{
    DeviceEvents, DeviceEventsHandler, Keycode, MouseButton, MousePosition,
};
use magnus::{Error, RHash, Ruby, Value};

/// An event captured from `device_query`, buffered until the Ruby listener
/// thread is ready to handle it.
enum RawEvent {
    KeyDown(String),
    KeyUp(String),
    MouseDown(usize),
    MouseUp(usize),
    MouseMove(i64, i64),
}

/// `device_query` only allows a single event loop to be initialised for the
/// lifetime of the process. The loop keeps polling in the background once it
/// has started, so we cache the first `DeviceEventsHandler` we manage to
/// create and reuse it for every `listen` call.
static HANDLER: OnceLock<Result<DeviceEventsHandler, String>> = OnceLock::new();

fn handler() -> Result<&'static DeviceEventsHandler, String> {
    HANDLER.get_or_init(|| {
        DeviceEventsHandler::new(Duration::from_millis(10))
            .ok_or_else(|| "device event loop is already running".to_string())
    })
    .as_ref()
    .map_err(|message| message.clone())
}

/// Converts a `Keycode`'s `Debug` variant name (e.g. `LControl`) into a
/// lowercase snake_case string (e.g. `l_control`).
fn keycode_to_name(keycode: Keycode) -> String {
    let variant = format!("{keycode:?}");
    let chars: Vec<char> = variant.chars().collect();
    let mut name = String::with_capacity(variant.len());

    for (index, character) in chars.iter().enumerate() {
        if character.is_ascii_uppercase() {
            let previous_is_upper = index > 0 && chars[index - 1].is_ascii_uppercase();
            let next_is_lower =
                index + 1 < chars.len() && chars[index + 1].is_ascii_lowercase();

            if index > 0 && (next_is_lower || !previous_is_upper) {
                name.push('_');
            }

            name.push(character.to_ascii_lowercase());
        } else {
            name.push(*character);
        }
    }

    name
}

/// Turns a buffered event into a Ruby `Hash` describing it.
fn event_to_ruby(ruby: &Ruby, event: RawEvent) -> Result<RHash, Error> {
    let hash = ruby.hash_new();

    match event {
        RawEvent::KeyDown(key) => {
            hash.aset("type", "key_down")?;
            hash.aset("key", key)?;
        }
        RawEvent::KeyUp(key) => {
            hash.aset("type", "key_up")?;
            hash.aset("key", key)?;
        }
        RawEvent::MouseDown(button) => {
            hash.aset("type", "mouse_down")?;
            hash.aset("button", button as i64)?;
        }
        RawEvent::MouseUp(button) => {
            hash.aset("type", "mouse_up")?;
            hash.aset("button", button as i64)?;
        }
        RawEvent::MouseMove(x, y) => {
            hash.aset("type", "mouse_move")?;
            hash.aset("x", x)?;
            hash.aset("y", y)?;
        }
    }

    Ok(hash)
}

/// Registers `device_query` callbacks that buffer events for the blocking
/// listener to drain. Each returned guard keeps its callback alive; dropping
/// them unregisters the callback.
#[allow(clippy::type_complexity)]
fn register_callbacks(
    handler: &DeviceEventsHandler,
    sender: &Sender<RawEvent>,
) -> (
    device_query::CallbackGuard<impl Fn(&Keycode) + Send + Sync + 'static>,
    device_query::CallbackGuard<impl Fn(&Keycode) + Send + Sync + 'static>,
    device_query::CallbackGuard<impl Fn(&MouseButton) + Send + Sync + 'static>,
    device_query::CallbackGuard<impl Fn(&MouseButton) + Send + Sync + 'static>,
    device_query::CallbackGuard<impl Fn(&MousePosition) + Send + Sync + 'static>,
) {
    let sender = Arc::new(Mutex::new(sender.clone()));

    let key_down_sender = Arc::clone(&sender);
    let key_down = handler.on_key_down(move |key: &Keycode| {
        let _ = key_down_sender
            .lock()
            .map(|sender| sender.send(RawEvent::KeyDown(keycode_to_name(*key))));
    });

    let key_up_sender = Arc::clone(&sender);
    let key_up = handler.on_key_up(move |key: &Keycode| {
        let _ = key_up_sender
            .lock()
            .map(|sender| sender.send(RawEvent::KeyUp(keycode_to_name(*key))));
    });

    let mouse_down_sender = Arc::clone(&sender);
    let mouse_down = handler.on_mouse_down(move |button: &MouseButton| {
        let _ = mouse_down_sender
            .lock()
            .map(|sender| sender.send(RawEvent::MouseDown(*button)));
    });

    let mouse_up_sender = Arc::clone(&sender);
    let mouse_up = handler.on_mouse_up(move |button: &MouseButton| {
        let _ = mouse_up_sender
            .lock()
            .map(|sender| sender.send(RawEvent::MouseUp(*button)));
    });

    let mouse_move_sender = Arc::clone(&sender);
    let mouse_move = handler.on_mouse_move(move |position: &MousePosition| {
        let _ = mouse_move_sender.lock().map(|sender| {
            sender.send(RawEvent::MouseMove(position.0 as i64, position.1 as i64))
        });
    });

    (key_down, key_up, mouse_down, mouse_up, mouse_move)
}

/// Blocks the calling Ruby thread, yielding a `Hash` describing each device
/// event (key down/up, mouse down/up/move) to the given block until it raises.
pub fn listen(ruby: &Ruby) -> Result<(), Error> {
    if !ruby.block_given() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "no block given",
        ));
    }

    let handler = handler().map_err(|message| {
        Error::new(ruby.exception_arg_error(), message)
    })?;
    let (sender, receiver): (Sender<RawEvent>, Receiver<RawEvent>) = mpsc::channel();

    let (_key_down, _key_up, _mouse_down, _mouse_up, _mouse_move) =
        register_callbacks(handler, &sender);

    loop {
        let event = match receiver.recv() {
            Ok(event) => event,
            Err(_) => break,
        };

        let hash = event_to_ruby(ruby, event)?;
        let _: Value = ruby.yield_value(hash)?;
    }

    Ok(())
}
