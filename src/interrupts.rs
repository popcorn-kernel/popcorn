use core::fmt::Arguments;
use core::panic::Location;
use core::panic::PanicInfo;
use crate::{clear_screen, print, println, serial_println, set_color};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use x86_64::instructions::segmentation::Segment;
use crate::gdt::{DOUBLE_FAULT_IST_INDEX, GDT};
use crate::vga_buffer::Color;
use pic8259::ChainedPics;
use spin;
use crate::misc::hlt_loop;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;


pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
lazy_static! {
    /// @brief The Interrupt Descriptor Table
    /// @details This is the Interrupt Descriptor Table, which contains the handlers for all interrupts.
    static ref IDT: InterruptDescriptorTable = {
       let mut idt = InterruptDescriptorTable::new();
        unsafe {
            // We need to ensure a fresh stack for double faults, or overflow faults
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
            idt.overflow.set_handler_fn(overflow_handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);
            idt.divide_error.set_handler_fn(division_handler);
            idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
            idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); // new
            idt.page_fault.set_handler_fn(page_fault_handler);
            idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        }

        idt
    };
}

/**
 * @brief Initializes the Interrupt Descriptor Table
 * @details This function initializes the Interrupt Descriptor Table, and loads it into the CPU.
 * This will allow us to handle interrupts.
 */
pub fn init_idt() {
    println!("Initializing IDT...");
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS};

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
    IDT.load();
}

/// Initializes the Programmable Interrupt Controller
pub fn init_pic() {
    println!("Initializing PIC...");
    unsafe {
        PICS.lock().initialize();
    }
}

/// Initializes things related to interrupts
pub fn init_interrupts() {
    println!("Initializing interrupts...");
    init_idt();
    init_pic();
    x86_64::instructions::interrupts::enable();
}

/**
 * @brief A struct containing technical information about a panic
 * @details This struct contains technical information about a panic, such as the instruction pointer, stack pointer, etc.
 * Fill this struct with the information you want to display when a panic occurs.
 */
pub struct PanicTechnicalInfo {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
    memory_address: u64,
    code: PageFaultErrorCode
}

/// @brief Implementation of the PanicTechnicalInfo struct
impl PanicTechnicalInfo {
    /**
     * @brief Creates a new PanicTechnicalInfo
     * @return A new PanicTechnicalInfo
     */
    fn new() -> Self {
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

/// @brief Handles a keyboard event, such as a key press
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    // TODO: Put scancode processing here

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

/// Handles any double faults. Double faults are caused by faults that occur while handling another fault.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    // Create a new PanicTechnicalInfo
    let mut panic_info = PanicTechnicalInfo::new();
    // Populate
    panic_info.instruction_pointer = stack_frame.instruction_pointer.as_u64();
    panic_info.code_segment = stack_frame.code_segment;
    panic_info.cpu_flags = stack_frame.cpu_flags;
    panic_info.stack_pointer = stack_frame.stack_pointer.as_u64();
    panic_info.stack_segment = stack_frame.stack_segment;

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "DOUBLE FAULT", &panic_info);
    panic!("Double fault");


}

/// @brief Processes an Overflow event (arithmetical)
extern "x86-interrupt" fn overflow_handler(
    stack_frame: InterruptStackFrame)
{
    // Create a new PanicTechnicalInfo
    let mut panic_info = PanicTechnicalInfo::new();
    // Populate
    panic_info.instruction_pointer = stack_frame.instruction_pointer.as_u64();
    panic_info.code_segment = stack_frame.code_segment;
    panic_info.cpu_flags = stack_frame.cpu_flags;
    panic_info.stack_pointer = stack_frame.stack_pointer.as_u64();
    panic_info.stack_segment = stack_frame.stack_segment;

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "ARITH OVERFLOW EXCEPTION", &panic_info);
}

/// @brief Processes a Division by Zero event
extern "x86-interrupt" fn division_handler(
    stack_frame: InterruptStackFrame)
{
    // Create a new PanicTechnicalInfo
    let mut panic_info = PanicTechnicalInfo::new();
    // Populate
    panic_info.instruction_pointer = stack_frame.instruction_pointer.as_u64();
    panic_info.code_segment = stack_frame.code_segment;
    panic_info.cpu_flags = stack_frame.cpu_flags;
    panic_info.stack_pointer = stack_frame.stack_pointer.as_u64();
    panic_info.stack_segment = stack_frame.stack_segment;

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "DIVISION EXCEPTION", &panic_info);
}

/// @brief Processes an Invalid Opcode event
extern "x86-interrupt" fn invalid_opcode_handler(
    stack_frame: InterruptStackFrame)
{
    // Create a new PanicTechnicalInfo
    let mut panic_info = PanicTechnicalInfo::new();
    // Populate
    panic_info.instruction_pointer = stack_frame.instruction_pointer.as_u64();
    panic_info.code_segment = stack_frame.code_segment;
    panic_info.cpu_flags = stack_frame.cpu_flags;
    panic_info.stack_pointer = stack_frame.stack_pointer.as_u64();
    panic_info.stack_segment = stack_frame.stack_segment;

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "INVALID OPCODE", &panic_info);
}

/// @brief Processes a Timer event. This is called every time the timer fires.
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{

    // Needs explicit EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// @brief Processes a Page Fault event
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;

    // Create a new PanicTechnicalInfo
    let mut panic_info = PanicTechnicalInfo::new();
    // Populate
    panic_info.instruction_pointer = stack_frame.instruction_pointer.as_u64();
    panic_info.code_segment = stack_frame.code_segment;
    panic_info.cpu_flags = stack_frame.cpu_flags;
    panic_info.stack_pointer = stack_frame.stack_pointer.as_u64();
    panic_info.stack_segment = stack_frame.stack_segment;
    panic_info.memory_address = Cr2::read().as_u64();
    panic_info.code = error_code;

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "PAGE FAULT", &panic_info);
}

/**
 * @brief Processes a Panic event
 * @details This function is called when a panic occurs. It prints the panic message, and halts the system.
 * @param info Information about the panic
 */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Create stack frame
    let mut stack_frame: PanicTechnicalInfo = PanicTechnicalInfo::new();

    // Fill stack tech info
    stack_frame.instruction_pointer = x86_64::registers::control::Cr2::read().as_u64();
    stack_frame.code_segment = x86_64::instructions::segmentation::CS::get_reg().0 as u64;
    stack_frame.cpu_flags = x86_64::registers::rflags::read_raw();
    stack_frame.stack_pointer = x86_64::registers::control::Cr2::read().as_u64();
    stack_frame.stack_segment = x86_64::instructions::segmentation::SS::get_reg().0 as u64;

    knl_panic(info.location().unwrap(), info.message().unwrap(), &stack_frame);
}

// Get number of newline characters in a string
fn get_newline_count(string: &str) -> usize {
    let mut count: usize = 0;
    for c in string.chars() {
        if c == '\n' {
            count += 1;
        }
    }
    count
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

    // If there's something weird, and it don't look good, who you gonna call?
    // PANIC BUSTERS!

    // FYI, when the system panics, you want to do the LEAST amount of unsafe functionality.
    // Try to keep things on the stack.

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
        stackFrame.stack_segment
        + stackFrame.stack_pointer
        + stackFrame.code_segment
            + stackFrame.cpu_flags
            + stackFrame.instruction_pointer;

    let title: &str = titles[(x % titles.len() as u64) as usize];
    let title_padding: usize = (80 / 2) - (title.len() / 2);

    // Render
    clear_screen!(Color::Magenta);

    set_color!(Color::White, Color::Magenta);
    // for every space in the title padding, print a space
    for _ in 0..title_padding {
        print!(" ");
    }

    set_color!(Color::Yellow, Color::Magenta);
    println!("{}", title);

    set_color!(Color::White, Color::Magenta);

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

    // Bios interrupt
    hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/*=======================================================
    TESTS
=======================================================*/

#[test_case]
fn test_init_interrupts() {
    init_interrupts();
    assert_eq!(IDT.load(), IDT_DESCRIPTOR);
}