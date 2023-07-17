#![no_std] // don't link the Rust standard library
#![feature(panic_info_message)]
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // re-export the test executor.
#![feature(custom_test_frameworks)] // use feature custom-test-frameworks.
#![test_runner(popcorn::test_runner)] // declare the test runner
#![feature(asm_const)]
#![feature(abi_x86_interrupt)]
#![feature(fmt_internals)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use popcorn::allocation::HEAP_SIZE;
use popcorn::memory::{BootInfoFrameAllocator, init_pagetable};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    popcorn::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_pagetable(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    popcorn::test_panic_handler(info)
}