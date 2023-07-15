/**
 * @file memory.rs
 * @brief Memory functions
 * @details This file contains functions for memory manipulation.
 */

/**
 * @brief Copy a block of memory
 * @param dst Pointer to the destination memory
 * @param src Pointer to the source memory
 * @param n Number of bytes to copy
 * @return Pointer to the destination memory
 */
#[no_mangle]
pub extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
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
pub extern "C" fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *dst.add(i) = c as u8;
        }
        i += 1;
    }
    dst
}