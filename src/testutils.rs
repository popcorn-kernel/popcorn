use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/**
 * @brief Panic handler for tests.
 * @details This function is called when a test panics.
 * @param info The panic information.
 */
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use crate::{ serial_print, serial_println};
use crate::system::task::hlt_loop;

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}