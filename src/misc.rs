use core::arch::asm;

/**
 * @brief Reboots the computer
 * @details This function reboots the computer by calling the BIOS interrupt 0x19.
 */
fn bios_interrupt_reboot() {
    x86_64::instructions::interrupts::disable();
    unsafe { asm!("int 0x19") };
}