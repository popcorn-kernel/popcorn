#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::testutils::test_runner)]
#![feature(panic_info_message)]
#![feature(fmt_internals)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use crate::system::task::hlt_loop;
use crate::system::vga_buffer::Color;
pub mod system;
pub mod testutils;
pub mod kernel;


/**
 * @brief Initializes the kernel.
 * @details This function initializes the kernel. Call this function before doing anything else.
 * To be used in Main, and to be used in Tests.
 */
pub fn init() {
    system::init_system();
}

/**
 * @brief Shuts down the kernel.
 * @details This function clears the screen, then shuts down the kernel, then shuts down the computer.
 * You should be able to call this anywhere if needed.
 */
pub fn shutdown() {
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Shutting down...");

    // Put deinitialization code here.

    // Stop processor
    system::task::hlt_loop();
}

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    use crate::kernel;
    kernel::init_kernel();
    test_main();
    hlt_loop();
}
