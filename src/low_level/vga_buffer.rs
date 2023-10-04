use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::low_level::vga_buffer::writer::Writer;
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

#[doc(hidden)]
pub fn print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn set_color(foreground: Color, background: Color) {
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(foreground, background);
    });
}

pub fn clear_screen(color: Color) {
    interrupts::without_interrupts(|| {
        WRITER.lock().clear_screen(color);
    });
}

pub fn backspace() {
    interrupts::without_interrupts(|| {
        WRITER.lock().backspace();
    });
}
pub fn cursor_back() {
    interrupts::without_interrupts(|| {
        WRITER.lock().cursor_back();
    });
}
pub fn cursor_front() {
    interrupts::without_interrupts(|| {
        WRITER.lock().cursor_front();
    });
}
