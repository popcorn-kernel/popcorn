#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(popcorn::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(panic_info_message)]
#![feature(fmt_internals)]

extern crate alloc;

use crate::vga_buffer::Color;
use core::panic::PanicInfo;

pub mod allocation;
pub mod gdt;
pub mod interrupt_handlers;
pub mod interrupts;
pub mod memory;
pub mod misc;
pub mod panic;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_interrupts();
}

/**
 * @brief Shuts down the kernel.
 * @details This function clears the screen, then shuts down the kernel, then shuts down the computer.
 */
pub fn shutdown() {
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Shutting down...");

    // Put deinitialization code here.

    // Shut down the kernel, and the computer.
    crate::hlt_loop();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
