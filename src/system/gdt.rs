use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

/// @brief The index of the double fault stack
/// @details This is the index of the double fault stack in the Interrupt Stack Table.
/// We need to ensure a fresh stack for double faults, or overflow faults
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    /// @brief The Task State Segment
    /// @details This is the Task State Segment, which contains the stack for various things.
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };

    /// @brief The Global Descriptor Table
    /// @details This is the Global Descriptor Table, which contains the segments for the CPU.
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub struct Selectors {
    pub(crate) code_selector: SegmentSelector,
    pub(crate) tss_selector: SegmentSelector,
}
