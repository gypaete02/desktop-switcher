use rdev::{Event, Key};

use lazy_static::lazy_static;

use std::sync::Mutex;

use crate::desktops::Desktops;

struct Input {
    /// A temporary counter to count how many times a key was pressed
    count: usize,
    is_alt_pressed: bool,
    is_super_pressed: bool,
    desktops: Desktops,
}

lazy_static! {
    static ref INPUT: Mutex<Input> = Mutex::new(Input {
        count: 0,
        is_alt_pressed: false,
        is_super_pressed: false,
        desktops: Desktops::new(),
    });
}

/// Start listening to input and executing movements. This function blocks until the end of the
/// program or if an error occurrs.
pub fn start_listening() -> Result<(), rdev::ListenError> {
    rdev::listen(callback)
}

fn callback(event: Event) {
    match event.event_type {
        rdev::EventType::KeyPress(key) => handle_key_press(key),
        rdev::EventType::KeyRelease(key) => handle_key_release(key),
        _ => (),
    }
}

fn handle_key_press(key: Key) {
    match key {
        Key::Alt => {
            INPUT.lock().unwrap().is_alt_pressed = true;
        }

        Key::MetaLeft => {
            INPUT.lock().unwrap().is_super_pressed = true;
        }

        Key::Tab => {
            let mut i = INPUT.lock().unwrap();
            if i.is_alt_pressed {
                i.count += 1;

                let count = i.count;
                i.desktops.preview_last(count);
            }
        }

        Key::LeftBracket => {
            let mut i = INPUT.lock().unwrap();
            if i.is_super_pressed {
                i.desktops.previous();
            }
        }

        Key::RightBracket => {
            let mut i = INPUT.lock().unwrap();
            if i.is_super_pressed {
                i.desktops.next();
            }
        }

        Key::Num1 => go_to_desktop(0),
        Key::Num2 => go_to_desktop(1),
        Key::Num3 => go_to_desktop(2),
        Key::Num4 => go_to_desktop(3),
        Key::Num5 => go_to_desktop(4),
        Key::Num6 => go_to_desktop(5),
        Key::Num7 => go_to_desktop(6),
        Key::Num8 => go_to_desktop(7),
        Key::Num9 => go_to_desktop(8),
        Key::Num0 => go_to_desktop(9),

        _ => {}
    }
}

fn handle_key_release(key: Key) {
    match key {
        Key::Alt => {
            let mut i = INPUT.lock().unwrap();
            let count = i.count;
            i.desktops.last(count);
            i.count = 0;
            i.is_alt_pressed = false;
        }

        Key::MetaLeft => {
            INPUT.lock().unwrap().is_super_pressed = false;
        }

        _ => (),
    }
}

fn go_to_desktop(index: usize) {
    let mut i = INPUT.lock().unwrap();
    if i.is_super_pressed {
        i.desktops.go_to(index)
    }
}
