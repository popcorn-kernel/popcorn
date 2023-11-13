#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

use bootloader::{entry_point, BootInfo};
#[allow(unused_imports)]
use popcorn::{
    error, hlt_loop, init, log,
    low_level::vga_buffer::{send_command_to_writer, Color, CommandToWriter},
    print_with_colors, println,
    userspace::output::MessageToVga,
    warn,
};
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    send_command_to_writer(CommandToWriter::ClearScreen(Color::Black));

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
