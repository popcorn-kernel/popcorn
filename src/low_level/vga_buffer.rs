use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::low_level::vga_buffer::writer::Writer;
mod buffer;
mod writer;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LighGrey = 0x07,
    DarkGrey = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    White = 0x0F,
}

const VGA_BUFFER: usize = 0xb8000;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> =
        Mutex::new(Writer::new(0, Color::Yellow, Color::Black, VGA_BUFFER,));
}
pub enum CommandToWriter<'a> {
    Print(fmt::Arguments<'a>),
    SetColor(Color, Color),
    ClearScreen(Color),
    Backspace,
    CursorBack,
    CursorFront,
}
pub fn send_command_to_writer(command: CommandToWriter) {
    interrupts::without_interrupts(|| {
        WRITER.lock().handle_command(command);
    });
}
