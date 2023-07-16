use core::arch::asm;

/**
 * @brief Halts the CPU on this process, freeing up the CPU for other processes.
 * @return Never
 */
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/**
 * @brief Reboots the computer
 * @details This function reboots the computer by calling the BIOS interrupt 0x19.
 */
fn bios_interrupt_reboot() {
    x86_64::instructions::interrupts::disable();
    unsafe { asm!("int 0x19") };
}