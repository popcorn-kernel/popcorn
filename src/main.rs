//popcorn 0.2.x

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
use core::panic::PanicInfo;
extern crate alloc;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

use bootloader::{entry_point, BootInfo};
use popcorn::{
    hlt_loop, init,
    low_level::vga_buffer::{clear_screen, Color},
    print_with_colors, println,
    userspace::output::MessageToVga, log, warn, error,
};
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    clear_screen(Color::Black);

    print_with_colors!(
        MessageToVga::new(Color::Yellow, Color::Black, "Welcome to the "),
        MessageToVga::new(Color::LightBlue, Color::Black, "Popcorn Kernel!")
    );
    println!(); //Newline being other than black and white caused a bug with the cursor
    log!("Initializing...");
    init(boot_info);
    log!("Initialized!");

    hlt_loop();
}
