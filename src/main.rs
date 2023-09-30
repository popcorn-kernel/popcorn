#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

use bootloader::{entry_point, BootInfo};
use popcorn::{init, println};
entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("hello world!");

    init();

    stack_overflow();

    println!("hello!");
    loop {}
}

fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}

