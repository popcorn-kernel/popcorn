#![feature(panic_info_message)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
//#![reexport_test_harness_main = "test_main"] // re-export the test executor.
//#![feature(custom_test_frameworks)] // use feature custom-test-frameworks.
//#![test_runner(crate::test_runner)] // declare the test runner


#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;


use core::arch::asm;
use core::panic::{PanicInfo};

mod vga_buffer;
mod memory;
use crate::vga_buffer::Color;

/**
 * @brief Processes a Panic event
 * @details This function is called when a panic occurs. It prints the panic message, and halts the system.
 * @param info Information about the panic
 */
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // If there's something weird, and it don't look good, who you gonna call?
    // PANIC BUSTERS!

    // FYI, when the system panics, you want to do the LEAST amount of unsafe functionality.
    clear_screen!(Color::DarkGray);

    set_color!(Color::White, Color::DarkGray);

    println!(include_str!("../locale/en_panic.txt"), info.location().unwrap(), info.message().unwrap());

    loop {}
}

/**
 * @brief Entry point
 * @details This function is called by the bootloader. It is the entry point of the kernel.
 * @return Never returns
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Your operating system initialization code goes here

    // Print some information
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Welcome to the Popcorn kernel!");

    // Rest of your operating system code
    loop {
    }
}

/**
 * @brief Shuts down the operating system
 * @details This function clears the screen, then shuts down the kernel, then shuts down the computer.
 */
pub fn shutdown()
{
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Shutting down...");

    // Put deinitialization code here.

    // Shut down the kernel, and the computer.
    unsafe { asm!("hlt"); }
}
