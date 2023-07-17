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
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use popcorn::{allocation, clear_screen, init, print, println, serial_println, set_color};
use popcorn::memory::{BootInfoFrameAllocator, init_pagetable};
use popcorn::vga_buffer::Color;
entry_point!(kernel_main);

/**
 * @brief The main function of the kernel
 * @details This function is called by the bootloader.
 * @param boot_info The boot information passed by the bootloader.
 */
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // This can be named arbitrarily.

    // Print some information
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Welcome to the Popcorn kernel!");
    serial_println!("Welcome to the Popcorn kernel!");

    // Initialize the kernel
    init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_pagetable(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocation::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // […] call `test_main` in test context
    println!("It did not crash!");

    // Halt until the next interrupt
    popcorn::hlt_loop();
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}