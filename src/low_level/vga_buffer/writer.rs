use super::Color;
use core::fmt;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const ACTUAL_BUFFER_WIDTH: usize = 50;
//Added because input stopped working after user tried to enter the 51 character.
//Probably qemu issue, maybe there is a way, but this is the temporary fix
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(
        column_position: usize,
        foreground: Color,
        background: Color,
        buffer: usize,
    ) -> Self {
        Writer {
            column_position,
            color_code: ColorCode::new(foreground, background),
            buffer: unsafe { &mut *(buffer as *mut Buffer) },
        }
    }
    pub fn move_cursor(&mut self, column_position: usize) {
        self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position + 1].invert_colors();
        if column_position == 0 {
            self.next_line();
        } else {
            self.column_position = column_position;
        }
        self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position + 1].invert_colors();
    }
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.column_position >= ACTUAL_BUFFER_WIDTH {
            self.move_cursor(0);
            return;
        }
        self.move_cursor(self.column_position + 1);
        self.set_char(byte);
    }
    fn set_char(&mut self, byte: u8) {
        self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position] = Char {
            ascii_character: byte,
            color_code: self.color_code,
        };
    }
    fn next_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            self.buffer.chars[row - 1] = self.buffer.chars[row]
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_screen(&mut self, color: Color) {
        let blank = Char {
            ascii_character: b' ',
            color_code: ColorCode::new(color, color),
        };
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col] = blank;
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
    pub fn backspace(&mut self) {
        if self.column_position == 0 {
            return;
        }
        self.set_char(b' ');
        self.move_cursor(self.column_position - 1);
    }
    pub fn cursor_back(&mut self) {
        if self.column_position == 0 {
            return;
        }
        self.move_cursor(self.column_position - 1)
    }
    pub fn cursor_front(&mut self) {
        if self.column_position == ACTUAL_BUFFER_WIDTH {
            return;
        }
        self.move_cursor(self.column_position + 1)
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        Self::generate(foreground as u8, background as u8)
    }
    fn generate(foreground: u8, background: u8) -> ColorCode {
        ColorCode((background) << 4 | (foreground))
    }
    fn get_colors(&self) -> (u8, u8) {
        (self.0 % 16u8, self.0 >> 4u8)
    }
    fn invert(&mut self) {
        let colors = self.get_colors();
        *self = Self::generate(colors.1, colors.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    ascii_character: u8,
    color_code: ColorCode,
}
impl Char {
    fn invert_colors(&mut self) {
        self.color_code.invert();
    }
}
#[repr(transparent)]
struct Buffer {
    chars: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
