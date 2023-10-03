//here goes proccessing the input from the user
//Backspace is implemented twice because even though it has a rawkey, its registered as Unicode.
//If in some case it would be a raw key, it would cause bugs
use crate::{
    low_level::vga_buffer::{backspace, cursor_back, cursor_front},
    print,
};

pub fn handle_keypress(key: char) {
    match key {
        '\u{8}' => backspace(),
        _ => print!("{}", key),
    }
}
use pc_keyboard::KeyCode;
pub fn handle_raw_keypress(key: KeyCode) {
    match key {
        KeyCode::Backspace => backspace(),
        KeyCode::LShift => {}
        KeyCode::RShift => {}
        KeyCode::CapsLock => {}
        KeyCode::ArrowLeft => cursor_back(),
        KeyCode::ArrowRight => cursor_front(),
        _ => print!("{:?}", key),
    }
}
