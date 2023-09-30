#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

use bootloader::{entry_point, BootInfo};
use popcorn::{init, print, println, hlt_loop, set_color, clear_screen, low_level::vga_buffer::Color};
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    clear_screen!(Color::Black);
    print!("Welcome to the ");
    set_color!(Color::LightBlue, Color::Black);
    println!("Popcorn Kernel!");
    init();

    hlt_loop();
}
