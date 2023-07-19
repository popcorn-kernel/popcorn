use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use lazy_static::lazy_static;
use vga::{
    colors::{Color16, DEFAULT_PALETTE},
    registers::{PlaneMask, WriteMode},
    vga::{VideoMode, VGA},
};
use vga::writers::{Graphics640x480x16, GraphicsWriter};
use crate::system::memory::memcpy;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const SIZE: usize = (WIDTH * HEIGHT) / 8;
const WIDTH_IN_BYTES: usize = WIDTH / 8;

/// Prints to the VGA text buffer through the global `WRITER` instance.
/// This macro ONLY accepts literal strings for the message.
/// Accepts variable number of arguments for formatting.
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::system::vga_video::_print(format_args!($($arg)*)));
}

/// Prints to the VGA text buffer through the global `WRITER` instance,
/// with a newline appended.
/// This macro ONLY accepts literal strings for the message.
/// Accepts variable number of arguments for formatting.
#[macro_export]
macro_rules! vga_println {
    () => ($crate::print!("\n"));
    ($($arg:expr),*) => {
        $crate::print!("{}\n", format_args!($($arg),*))
    };
}

/// Clears the screen with the given color.
/// Accepts a `Color` enum value.
/// See the `Color` enum for available colors.
#[macro_export]
macro_rules! vga_clear_screen {
    ($arg:expr) => {
        $crate::system::vga_video::clear_screen($arg);
    };
}


lazy_static! {
    pub static ref VGA_WRITER: Graphics640x480x16 = Graphics640x480x16::new();
}

pub fn clear_screen(color: Color16) {
    VGA_WRITER.clear_screen(color);
}

pub fn blit_buffer(buffer: &VGABuffer, x: usize, y: usize) {
    unsafe {
        let mut vga = VGA.lock();
        vga.graphics_controller_registers
            .set_write_mode(WriteMode::Mode2);
        vga.graphics_controller_registers.set_bit_mask(0xFF);
        vga.sequencer_registers
            .set_plane_mask(PlaneMask::ALL_PLANES);

        let frame_buffer = usize::from(vga.get_frame_buffer()) as *mut u8;
        for yy in 0..buffer.h {
            for xx in 0..buffer.w {
                unsafe {
                    let pixel_mask = 0x80 >> (xx & 0x07);
                    vga.graphics_controller_registers
                        .set_bit_mask(pixel_mask);
                    let offset = xx / 8 + yy * WIDTH_IN_BYTES;
                    frame_buffer.add(offset).read_volatile();
                    frame_buffer.add(offset).write_volatile(buffer.buffer[xx + yy * buffer.w]);
                }
            }

        }

    }
}

pub struct VGABuffer {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<u8>,
}

pub fn init_vga() {
    VGA_WRITER.set_mode();
    clear_screen(Color16::Green);
    // Set VGA White color... To white, the above changes it to PINK, of all colors...

}

impl VGABuffer {
    pub fn new(x: usize, y: usize, color: Color16) -> VGABuffer {
        let mut vga = VGABuffer {
            w: x,
            h: y,
            buffer: vec![0; (x*y)],
        };

        // Fill with color. Each byte contains two pixels. Bit 0-3 is the left pixel, 4-7 is the right pixel.
        for i in 0..(x*y) {
            // Copy, and mask out the left 4 bits
            vga.buffer[i] = color as u8;
        }

        vga

    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color16) {
        let pos = x + y * self.w;
        if(pos >= self.buffer.len()) {
            return;
        }
        // Each byte contains two pixels. Bit 0-3 is the left pixel, 4-7 is the right pixel.
        self.buffer[pos] = color as u8;

    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.buffer[x + y * self.w]
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: Color16)
    {
        // Bresenham's line algorithm
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let mut x2 = x2 as isize;
        let mut y2 = y2 as isize;

        let mut dx = (x2 - x1).abs();
        let mut dy = -(y2 - y1).abs();

        let mut sx = if x1 < x2 { 1 } else { -1 };
        let mut sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.draw_pixel(x1 as usize, y1 as usize, color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x1 += sx;
            }

            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }


    pub fn blit_buffer(&mut self, x: usize, y: usize, other: &mut VGABuffer) {
        // Blit the buffer to this buffer at the specified position.
        // If the buffer is too large, it will be clipped.

        // Calculate the maximum x and y values and row length of the other buffer
        let max_x = min(x + other.w, self.w);
        let max_y = min(y + other.h, self.h);
        let other_row_length = other.w;

        // Copy each row of the other buffer to the corresponding row in self.buffer
        for row in 0..(max_y - y) {
            let self_row = y + row;
            let other_row = row;

            // Perform the memory copy
            self.buffer[self_row + x..max_x * self.w].copy_from_slice(&other.buffer[other_row + 0..(max_x - x) * self.h]);
        }
    }

}