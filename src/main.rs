#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use bootloader::{entry_point, BootInfo};
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    loop {
        println!("hello world!");
    }
}
