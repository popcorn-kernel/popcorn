/**
 * @brief Halts the CPU on this task.
 * @details This function halts the CPU on this task, freeing up the CPU for other tasks.
 * If you don't use this, you can wind up with a CPU that's running at 100% usage,
 * as it loops endlessly.
 */
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}