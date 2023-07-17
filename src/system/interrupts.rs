use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::instructions::segmentation::Segment;
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::println;
use crate::system::gdt::{DOUBLE_FAULT_IST_INDEX, GDT};
use crate::system::interrupt_handlers;

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
    pub(crate) fn as_u8(self) -> u8 {
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

            // We need to ensure a fresh stack for double faults
            idt.double_fault.set_handler_fn(interrupt_handlers::double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
            idt.overflow.set_handler_fn(interrupt_handlers::overflow_handler);
            idt.divide_error.set_handler_fn(interrupt_handlers::division_handler);
            idt.invalid_opcode.set_handler_fn(interrupt_handlers::invalid_opcode_handler);
            idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(interrupt_handlers::timer_interrupt_handler); // new
            idt.page_fault.set_handler_fn(interrupt_handlers::page_fault_handler);
            idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(interrupt_handlers::keyboard_interrupt_handler);
        }

        idt
    };
}

/**
 * @brief Initializes the Global Descriptor Table
 * @details This function initializes the Global Descriptor Table, and loads it into the CPU.
 * This will allow us to use the GDT.
 */
pub fn init_gdt() {
    println!("Initializing GDT...");
    use x86_64::instructions::segmentation::CS;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

/**
 * @brief Initializes the Interrupt Descriptor Table
 * @details This function initializes the Interrupt Descriptor Table, and loads it into the CPU.
 * This will allow us to handle interrupts.
 */
pub fn init_idt() {
    println!("Initializing IDT...");

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
    init_gdt();
    init_idt();
    init_pic();
    x86_64::instructions::interrupts::enable();
}
