// Use this file for exporting functions from the System directory.

pub mod allocation;
pub mod gdt;
pub mod interrupt_handlers;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod power;
pub mod serial;
pub mod vga_buffer;
pub mod task;

/// @brief Initializes the system's hardware, such as the GDT, IDT, etc.
pub fn init_system()
{
    interrupts::init_interrupts();
}

/// @brief Make sure the system is properly set up.
pub fn validate_system() -> bool
{
    // TODO: Add more validation checks
    true
}