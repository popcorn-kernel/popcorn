#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
extern crate alloc;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

use bootloader::{entry_point, BootInfo};
use popcorn::{
    clear_screen, hlt_loop, init,
    low_level::vga_buffer::{Color, MessageToVga},
     print_with_colors, println, set_color,
};
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    clear_screen!(Color::Black);

    print_with_colors!(
        MessageToVga::new(Color::White, Color::Black, "Welcome to the "),
        MessageToVga::new(Color::LightBlue, Color::Black, "Popcorn Kernel!\n")
    );

    set_color!(Color::White, Color::Black);

    init(boot_info);

    hlt_loop();
}
