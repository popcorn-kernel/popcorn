// Import the kernel directory, and offer some functions to manage it from the outside

use crate::system::task::hlt_loop;
use crate::system::validate_system;
use crate::system::vga_buffer::Color;
use crate::{print, println, set_color};

pub mod kernel_main;

/**
 * @brief Call this to start the kernel, once low-level initialization is done
 * @details Call this to start the kernel, once low-level initialization is done.
 * Don't call this function before low-level initialization is done, or you will get problems.
 */
pub fn init_kernel() {
    // Make sure the system is properly set up first. Could risk a bad time otherwise.
    if !validate_system() {
        set_color!(Color::Red, Color::Black);
        println!("System validation failed. Halting.");
        set_color!(Color::White, Color::Black);
        hlt_loop();
    }
    set_color!(Color::Green, Color::Black);
    println!("System validation passed. Starting kernel...");
    set_color!(Color::White, Color::Black);
    print!("Welcome to ");
    set_color!(Color::LightCyan, Color::Black);
    print!("TRANS/Popcorn");
    set_color!(Color::White, Color::Black);
    println!("!");
    kernel_main::main();
}
