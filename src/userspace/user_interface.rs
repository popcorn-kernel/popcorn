//here goes proccessing the input from the user
//Backspace is implemented twice because even though it has a rawkey, its registered as Unicode.
//If in some case it would be a raw key, it would cause bugs
use crate::{
    low_level::vga_buffer::{send_command_to_writer, CommandToWriter},
    print,
};

pub fn handle_keypress(key: char) {
    match key {
        '\u{8}' => send_command_to_writer(CommandToWriter::Backspace),
        _ => print!("{}", key),
    }
}
use pc_keyboard::KeyCode;
pub fn handle_raw_keypress(key: KeyCode) {
    match key {
        KeyCode::Backspace => send_command_to_writer(CommandToWriter::Backspace),
        KeyCode::LShift => {}
        KeyCode::RShift => {}
        KeyCode::CapsLock => {}
        KeyCode::ArrowLeft => send_command_to_writer(CommandToWriter::CursorBack),
        KeyCode::ArrowRight => send_command_to_writer(CommandToWriter::CursorFront),
        _ => print!("{:?}", key),
    }
}
