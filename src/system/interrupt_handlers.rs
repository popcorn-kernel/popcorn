use crate::println;
use crate::system::interrupts::{InterruptIndex, PICS};
use crate::system::panic::{knl_panic_str, PanicTechnicalInfo};
use core::arch::asm;
use core::panic::Location;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

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

pub extern "x86-interrupt" fn syscall_handler(_stack_frame: InterruptStackFrame) {
    // Grab registers and print to screen
    let rax: u64;
    let rbx: u64;
    let rcx: u64;
    let rdx: u64;
    let rsi: u64;
    let rdi: u64;
    // Get registers
    unsafe {
        asm!("mov {o}, rax", o = out(reg) rax);
        asm!("mov {o}, rbx", o = out(reg) rbx);
        asm!("mov {o}, rcx", o = out(reg) rcx);
        asm!("mov {o}, rdx", o = out(reg) rdx);
        asm!("mov {o}, rsi", o = out(reg) rsi);
        asm!("mov {o}, rdi", o = out(reg) rdi);
    }
    println!("RAX: {}", rax);
    println!("RBX: {}", rbx);
    println!("RCX: {}", rcx);
    println!("RDX: {}", rdx);
    println!("RSI: {}", rsi);
    println!("RDI: {}", rdi);
    println!("Syscall");
}

/// Handles any double faults. Double faults are caused by faults that occur while handling another fault.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    // Create a new PanicTechnicalInfo
    let panic_info = PanicTechnicalInfo {
        instruction_pointer: stack_frame.instruction_pointer.as_u64(),
        code_segment: stack_frame.code_segment,
        cpu_flags: stack_frame.cpu_flags,
        stack_pointer: stack_frame.stack_pointer.as_u64(),
        stack_segment: stack_frame.stack_segment,
        ..Default::default()
    };

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "DOUBLE FAULT", &panic_info);
    panic!("Double fault");
}

/// @brief Processes an Overflow event (arithmetical)
pub extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    // Create a new PanicTechnicalInfo
    let panic_info = PanicTechnicalInfo {
        instruction_pointer: stack_frame.instruction_pointer.as_u64(),
        code_segment: stack_frame.code_segment,
        cpu_flags: stack_frame.cpu_flags,
        stack_pointer: stack_frame.stack_pointer.as_u64(),
        stack_segment: stack_frame.stack_segment,
        ..Default::default()
    };

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "ARITH OVERFLOW EXCEPTION", &panic_info);
}

/// @brief Processes a Division by Zero event
pub extern "x86-interrupt" fn division_handler(stack_frame: InterruptStackFrame) {
    // Create a new PanicTechnicalInfo
    let panic_info = PanicTechnicalInfo {
        instruction_pointer: stack_frame.instruction_pointer.as_u64(),
        code_segment: stack_frame.code_segment,
        cpu_flags: stack_frame.cpu_flags,
        stack_pointer: stack_frame.stack_pointer.as_u64(),
        stack_segment: stack_frame.stack_segment,
        ..Default::default()
    };

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "DIVISION EXCEPTION", &panic_info);
}

/// @brief Processes an Invalid Opcode event
pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    // Create a new PanicTechnicalInfo
    let panic_info = PanicTechnicalInfo {
        instruction_pointer: stack_frame.instruction_pointer.as_u64(),
        code_segment: stack_frame.code_segment,
        cpu_flags: stack_frame.cpu_flags,
        stack_pointer: stack_frame.stack_pointer.as_u64(),
        stack_segment: stack_frame.stack_segment,
        ..Default::default()
    };

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
    let panic_info = PanicTechnicalInfo {
        instruction_pointer: stack_frame.instruction_pointer.as_u64(),
        code_segment: stack_frame.code_segment,
        cpu_flags: stack_frame.cpu_flags,
        stack_pointer: stack_frame.stack_pointer.as_u64(),
        stack_segment: stack_frame.stack_segment,
        memory_address: Cr2::read().as_u64(),
        code: error_code,
    };

    // Create arguments for the panic
    knl_panic_str(Location::caller(), "PAGE FAULT", &panic_info);
}
