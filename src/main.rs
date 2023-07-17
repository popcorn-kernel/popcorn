#![no_std] // don't link the Rust standard library
#![feature(panic_info_message)]
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // re-export the test executor.
#![feature(custom_test_frameworks)] // use feature custom-test-frameworks.
#![test_runner(popcorn::testutils::test_runner)] // declare the test runner
#![feature(asm_const)]
#![feature(abi_x86_interrupt)]
#![feature(fmt_internals)]

extern crate alloc;

#[cfg(not(test))]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use popcorn::{kernel};
use popcorn::system::{allocation, init_system, task};
#[cfg(not(test))]
use popcorn::system::memory::{BootInfoFrameAllocator, init_pagetable};


#[cfg(not(test))]
entry_point!(kernel_main);

/**
 * @brief The main function of the kernel
 * @details This function is called by the bootloader.
 * @param boot_info The boot information passed by the bootloader.
 */
#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use popcorn::system::vga_buffer::Color;
    use popcorn::{ clear_screen, println, set_color};
    use x86_64::VirtAddr;

    // This can be named arbitrarily.

    // Print some information
    clear_screen!(Color::Black);
    set_color!(Color::White, Color::Black);
    println!("Initializing hardware...");

    // Initialize the kernel
    init_system();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_pagetable(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocation::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // The heap is now ready to be used. We can now use Box, Vec, etc.

    kernel::init_kernel();

    // Halt until the next interrupt
    task::hlt_loop();
}

// The below needs to be separate from lib.rs, so it doesn't end up in tests.
/**
 * @brief Processes a Panic event
 * @details This function is called when a panic occurs. It prints the panic message, and halts the system.
 * @param info Information about the panic
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use popcorn::system::panic::{knl_panic, PanicTechnicalInfo};
    use x86_64::instructions::segmentation::Segment;
    // Create stack frame
    let mut stack_frame: PanicTechnicalInfo = PanicTechnicalInfo::new();

    // Fill stack tech info
    stack_frame.instruction_pointer = x86_64::registers::control::Cr2::read().as_u64();
    stack_frame.code_segment = x86_64::instructions::segmentation::CS::get_reg().0 as u64;
    stack_frame.cpu_flags = x86_64::registers::rflags::read_raw();
    stack_frame.stack_pointer = x86_64::registers::control::Cr2::read().as_u64();
    stack_frame.stack_segment = x86_64::instructions::segmentation::SS::get_reg().0 as u64;

    knl_panic(
        info.location().unwrap(),
        info.message().unwrap(),
        &stack_frame,
    );
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    popcorn::testutils::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

/*'d like to interject for a moment: what you are referring to as Popcorn, is in fact,
 * TRANS/Popcorn, or as I have recetly taken to calling it, TRANS plus Popcorn */
