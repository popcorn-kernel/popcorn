use core::fmt::Arguments;
use core::panic::Location;
use x86_64::structures::idt::PageFaultErrorCode;
use crate::{clear_screen, hlt_loop, print, println, serial_println, set_color};
use crate::vga_buffer::Color;

// If there's something weird, and it don't look good, who you gonna call?

// FYI, when the system panics, you want to do the LEAST amount of unsafe functionality.
// I recommend The Power of Ten, Rules for Developing Safety-Critical Code, by Gerard J. Holzmann at NASA.

// 1. Avoid complex flow constructs, such as goto and recursion.
// 2. All loops must have fixed bounds.
// 3. Avoid heap memory allocation.
// 4. Restrict functions to a single printed page (60 lines).
// 5. Use a minimum of two runtime assertions per function.
// 6. Restrict the scope of data to the minimum possible.
// 7. Check the return value of all non-void functions, or cast to void to indicate that the return value is ignored.
// 8. Use the preprocessor sparingly.
// 9. Limit pointer use to a single dereference, or declare them const or volatile.
// 10. All code must be compiled with all compiler warnings enabled at the compiler’s most pedantic setting.

/**
 * @brief A struct containing technical information about a panic
 * @details This struct contains technical information about a panic, such as the instruction pointer, stack pointer, etc.
 * Fill this struct with the information you want to display when a panic occurs.
 */
pub struct PanicTechnicalInfo {
    pub(crate) instruction_pointer: u64,
    pub(crate) code_segment: u64,
    pub(crate) cpu_flags: u64,
    pub(crate) stack_pointer: u64,
    pub(crate) stack_segment: u64,
    pub(crate) memory_address: u64,
    pub(crate) code: PageFaultErrorCode
}

/// @brief Implementation of the PanicTechnicalInfo struct
impl PanicTechnicalInfo {
    /**
     * @brief Creates a new PanicTechnicalInfo
     * @return A new PanicTechnicalInfo
     */
    pub(crate) fn new() -> Self {
        Self {
            instruction_pointer: 0,
            code_segment: 0,
            cpu_flags: 0,
            stack_pointer: 0,
            stack_segment: 0,
            memory_address: 0,
            code: PageFaultErrorCode::empty()
        }
    }
}

/// Location, str, techinfo overload, helps with convenience
pub fn knl_panic_str(location: &Location, message: &'static str, stackFrame: &PanicTechnicalInfo)
{
    let x = &[message];
    // Create arguments for the panic
    let args = Arguments::new_v1(
        x, &match () {
            () => [],
        });
    knl_panic(Location::caller(), &args, stackFrame)
}

/// Get the title the Kernel Panic should use.
fn knl_panic_get_title(stack_frame: &PanicTechnicalInfo) -> &'static str
{
    // Randomly select a title, for added fun
    let titles: [&str; 12] = [
        // Silly titles
        "Pop goes the kernel!",
        "Popcorn Pandemonium!",
        "Kernel Kablooey!",
        "Kernel Kablooey 2: Electric Boogaloo!",
        "Kernel Kablooey 3: The Reckoning!",
        "MEDIC!", // The Scout, Team Fortress 2
        "Oh, fiddlesticks, what now?", // Doctor Kleiner, Half-Life 2
        "Doc, come on, man!", // The Scout, Team Fortress 2
        "Don't Panic!", // The Hitchhiker's Guide to the Galaxy
        "Kernel Panic!",
        "Kernel Panic! (Not Clickbait)",
        "Kernel Panic! (Gone Wrong)",
    ];

    // Safest way to get a random number is to add up all the stack frame values, and use that as the seed.
    let mut x: u64 =
        stack_frame.stack_segment
            + stack_frame.stack_pointer
            + stack_frame.code_segment
            + stack_frame.cpu_flags
            + stack_frame.instruction_pointer;

    let title: &str = titles[(x % titles.len() as u64) as usize];
    &title
}

fn knl_panic_print(title: &str, location: &Location, message: &Arguments, stackFrame: &PanicTechnicalInfo)
{
    let title_padding: usize = (80 / 2) - (title.len() / 2);

    // Render
    clear_screen!(Color::Green);

    set_color!(Color::White, Color::Green);
    // for every space in the title padding, print a space
    for _ in 0..title_padding {
        print!(" ");
    }

    set_color!(Color::Yellow, Color::Green);
    println!("{}", title);

    set_color!(Color::White, Color::Green);

    println!(
        include_str!("../locale/en_panic.txt"),
        location,
        message,
        stackFrame.instruction_pointer,
        stackFrame.code_segment,
        stackFrame.cpu_flags,
        stackFrame.stack_pointer,
        stackFrame.stack_segment,
        stackFrame.memory_address,
        stackFrame.code
    );
}

/**
 * @brief Processes a Panic event, this one can be called from anywhere
 * @details This function is called when a panic occurs. It prints the panic message, and halts the system.
 * @param location Information about the location of the panic
 * @param message The message to print
 * @param stackFrame Information about the stack frame, such as the instruction pointer, stack pointer, etc.
 */
pub fn knl_panic(location: &Location, message: &Arguments, stackFrame: &PanicTechnicalInfo) -> ! {
    println!("KERNEL PANIC!");

    // Print stuff to Serial
    serial_println!("KERNEL PANIC!");
    serial_println!("Location: {}", location);
    serial_println!("Message: {}", message);

    let title: &str = knl_panic_get_title(stackFrame);
    // Check result, if empty or null, use default title
    if title.is_empty() {
        knl_panic_print("Kernel Panic!", location, message, stackFrame);
    }
    else {
        knl_panic_print(title, location, message, stackFrame);
    }

    // TODO: Restart the system
    hlt_loop();
}