use bootloader::bootinfo::MemoryMap;
/**
 * @file memory.rs
 * @brief Memory functions
 * @details This file contains functions for memory manipulation and management.
 */
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Page, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

use bootloader::bootinfo::MemoryRegionType;
use x86_64::registers::control::Cr3;

impl BootInfoFrameAllocator {
    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// Creates an example mapping for the given page to frame `0xb8000`.
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/**
 * @brief Copy a block of memory
 * @param dst Pointer to the destination memory
 * @param src Pointer to the source memory
 * @param n Number of bytes to copy
 * @return Pointer to the destination memory
 */
#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
        i += 1;
    }
    dst
}

/**
 * @brief Set a block of memory to a value
 * @param dst Pointer to the memory to set
 * @param c Value to set the memory to
 * @param n Number of bytes to set
 * @return Pointer to the memory
 */
#[no_mangle]
pub unsafe extern "C" fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *dst.add(i) = c as u8;
        }
        i += 1;
    }
    dst
}

/**
 * @brief Compare two blocks of memory
 * @param s1 Pointer to the first block of memory
 * @param s2 Pointer to the second block of memory
 * @param n Number of bytes to compare
 * @return 0 if the blocks are equal, nonzero otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        unsafe {
            if *s1.add(i) != *s2.add(i) {
                return 1;
            }
        }
        i += 1;
    }
    0
}

/** ACTIVE_LV4_TABLE
 * @brief Get a mutable reference to the active level 4 page table
 * @param phys_mem_offset The offset of the physical memory
 * @return A mutable reference to the active level 4 page table
 */
pub unsafe fn active_lv4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (lv4_table_frame, _) = Cr3::read();

    let phys = lv4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
/// Translates the given virtual address to the mapped physical address, or `None` if the address is not mapped.
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

/// @brief Private function that is called by `translate_addr`.
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

use x86_64::structures::paging::page_table::FrameError;
use x86_64::structures::paging::{OffsetPageTable, PageTable};

/// Initialize a new OffsetPageTable.
pub unsafe fn init_pagetable(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_lv4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
