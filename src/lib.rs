#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use bootloader::BootInfo;
use low_level::{
    allocator, gdt, interrupts,
    memory::{self, PopFrameAllocator},
};
use x86_64::VirtAddr;

pub mod low_level;
pub mod userspace;
pub fn init(boot_info: &'static BootInfo) {
    initialize_gdt_and_interrupts();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { PopFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
fn initialize_gdt_and_interrupts() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
