use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;
use crate::serial_print;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

// Convert u8 to Color
pub fn convert_to_color(val: u8) -> Color
{
    // Convert the u8 to a Color
    match val {
        0x0 => Color::Black,
        0x1 => Color::Blue,
        0x2 => Color::Green,
        0x3 => Color::Cyan,
        0x4 => Color::Red,
        0x5 => Color::Magenta,
        0x6 => Color::Brown,
        0x7 => Color::LightGray,
        0x8 => Color::DarkGray,
        0x9 => Color::LightBlue,
        0xA => Color::LightGreen,
        0xB => Color::LightCyan,
        0xC => Color::LightRed,
        0xD => Color::Pink,
        0xE => Color::Yellow,
        0xF => Color::White,
        _ => Color::Black,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUF_WIDTH {
                    self.new_line();
                }
                let row = BUF_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn set_cursor_pos(&mut self, row: usize, col: usize) {
        if row >= BUF_HEIGHT || col >= BUF_WIDTH {
            return; // Invalid position, do nothing
        }
        self.column_position = col;
    }

    pub fn clear_screen(&mut self, color: Color) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(color, color),
        };

        for row in 0..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                self.buffer.chars[row][col].write(blank);
            }
        }

        self.set_cursor_pos(0, 0);
    }

    fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
    fn new_line(&mut self) {
        for row in 0..BUF_HEIGHT - 1 {
            for col in 0..BUF_WIDTH {
                let character = self.buffer.chars[row + 1][col].read();
                self.buffer.chars[row][col].write(character);
            }
        }
        self.clear_row(BUF_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUF_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// macros

/// Prints to the VGA text buffer through the global `WRITER` instance.
/// This macro ONLY accepts literal strings for the message.
/// Accepts variable number of arguments for formatting.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::system::vga_buffer::_print(format_args!($($arg)*)));
}

/// Prints to the VGA text buffer through the global `WRITER` instance,
/// with a newline appended.
/// This macro ONLY accepts literal strings for the message.
/// Accepts variable number of arguments for formatting.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:expr),*) => {
        $crate::print!("{}\n", format_args!($($arg),*))
    };
}

/// Clears the screen with the given color.
/// Accepts a `Color` enum value.
/// See the `Color` enum for available colors.
#[macro_export]
macro_rules! clear_screen {
    ($arg:expr) => {
        $crate::system::vga_buffer::_clear_screen($arg);
    };
}

/// Sets the foreground and background color for the VGA text buffer.
/// Accepts two `Color` enum values.
/// See the `Color` enum for available colors.
/// The first argument is the foreground color, the second is the background color.
#[macro_export]
macro_rules! set_color {
    ($foreground:expr, $background:expr) => {
        $crate::system::vga_buffer::_set_color($foreground, $background)
    };
}

// private functions

/**
 *  \brief Prints to the VGA text buffer through the global `WRITER` instance.
 *  This function is called by the `print!` macro.
 *  \param args The arguments to print.
 */
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        use core::fmt::Write;
        WRITER.lock().write_fmt(args).unwrap();
        serial_print!("{}", args);
    });
}

/**
 *  \brief Clears the screen with the given color.
 *  This function is called by the `clear_screen!` macro.
 *  \param color The color to clear the screen with.
 */
#[doc(hidden)]
pub fn _clear_screen(color: Color) {
    interrupts::without_interrupts(|| {
        WRITER.lock().clear_screen(color);
    });
}

/**
 *  \brief Sets the foreground and background color for the VGA text buffer.
 *  This function is called by the `set_color!` macro.
 *  \param foreground The foreground color.
 *  \param background The background color.
 */
#[doc(hidden)]
pub fn _set_color(foreground: Color, background: Color) {
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(foreground, background);
    });
}
