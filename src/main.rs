#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // re-export the test executor.
#![feature(custom_test_frameworks)] // use feature custom-test-frameworks.
#![test_runner(crate::test_runner)] // declare the test runner

use core::panic::PanicInfo;
#[macro_use]
extern crate popcorn;
mod acpi;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    libc_println!("{}", info);
    loop {} // we need a less resource intensive pause mechanism
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
