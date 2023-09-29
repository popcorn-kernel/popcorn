#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(popcorn::testutils::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use popcorn::system::allocation;
use popcorn::system::allocation::HEAP_SIZE;
use popcorn::system::memory::{init_pagetable, BootInfoFrameAllocator};
use popcorn::testutils::QemuExitCode;
use popcorn::{serial_println, testutils};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    serial_println!("Heap allocation test");

    popcorn::init();
    let addr = boot_info.physical_memory_offset;
    let phys_mem_offset = VirtAddr::new(addr);
    let mut mapper = unsafe { init_pagetable(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocation::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();

    testutils::exit_qemu(QemuExitCode::Success);
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
    testutils::test_panic_handler(info)
}
