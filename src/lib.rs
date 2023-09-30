#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]

use low_level::{gdt, interrupts};

pub mod low_level;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}