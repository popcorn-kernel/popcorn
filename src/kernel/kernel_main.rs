use crate::{println, system};
use core::arch::asm;

/// @brief The main function of the kernel
pub fn main() {
    // The heap should be initialized by now, along with everything we need to get started.
    println!("Hello, x86!");
    println!("int 0x80 test, expect to see 'Syscall' with RAX equal to 42:");
    unsafe { asm!("mov rax, 42", "int 0x80") };
    //println!("syscall test, expect to see 'Syscall':");
    //unsafe{asm!("syscall")};

    loop {}

    system::task::hlt_loop();
}
