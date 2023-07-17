use core::panic::Location;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::system::interrupts::{InterruptIndex, PICS};
use crate::system::panic::{knl_panic_str, PanicTechnicalInfo};

/// @brief Handles a keyboard event, such as a key press
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let _scancode: u8 = unsafe { port.read() };
    // TODO: Put scancode processing here

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

/// Handles any double faults. Double faults are caused by faults that occur while handling another fault.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
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
pub extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
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
pub extern "x86-interrupt" fn division_handler(stack_frame: InterruptStackFrame) {
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
pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
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
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Needs explicit EOI
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// @brief Processes a Page Fault event
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
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
