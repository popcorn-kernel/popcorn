#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]

use low_level::{interrupts, gdt};

pub mod low_level;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
