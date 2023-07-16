#![feature(panic_info_message)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // re-export the test executor.
#![feature(custom_test_frameworks)] // use feature custom-test-frameworks.
#![test_runner(crate::test_runner)] // declare the test runner
#![feature(asm_const)]
#![feature(abi_x86_interrupt)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

use core::arch::asm;
use crate::misc::hlt_loop;

pub mod interrupts;

mod vga_buffer;
mod memory;
mod gdt;
use crate::vga_buffer::Color;
mod serial;
mod misc;

pub fn init() {
    // Set blink text mode to ON
/*
; Read I/O Address 0x03DA to reset index/data flip-flop
	mov dx, 0x03DA
	in al, dx
	; Write index 0x30 to 0x03C0 to set register index to 0x30
	mov dx, 0x03C0
	mov al, 0x30
	out dx, al
	; Read from 0x03C1 to get register contents
	inc dx
	in al, dx
	; Set Bit 3 to enable Blink
	or al, 0x08
	; Write to 0x03C0 to update register with changed value
	dec dx
	out dx, al
 */
unsafe {
        asm!("
        mov dx, 0x03DA
        in al, dx
        mov dx, 0x03C0
        mov al, 0x30
        out dx, al
        inc dx
        in al, dx
        or al, 0x08
        dec dx
        out dx, al
        ");
    }

    interrupts::init_interrupts();
}

/**
 * @brief Entry point
 * @details This function is called by the bootloader. It is the entry point of the kernel.
 * @return Never returns
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {

    // Print some information
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Welcome to the Popcorn kernel!");
    serial_println!("Welcome to the Popcorn kernel!");

    // Initialize the kernel
    init();

    // Halt until the next interrupt
    hlt_loop();
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
    hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    libc_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert!(1 == 1);
    libc_println!("[ok]");
}