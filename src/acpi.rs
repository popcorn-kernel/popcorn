// i don't know what these actually mean it was LLM generated.

use x86_64::instructions::port::Port;

const CMOS_ADDR_PORT: u16 = 0x70;
const CMOS_DATA_PORT: u16 = 0x71;

const CMOS_STATUS_A: u8 = 0x0A;
const CMOS_STATUS_B: u8 = 0x0B;

const RTC_REGISTER_B_MASK: u8 = 0x02;
const RTC_NMI_DISABLE_MASK: u8 = 0x80;

const ACPI_OFF: u8 = 0x02;

fn out_cmos_register(register: u8, value: u8) {
    let mut port = Port::new(CMOS_ADDR_PORT);
    unsafe {
            port.write(register);
    }

    let mut port = Port::new(CMOS_DATA_PORT);
    unsafe {
            port.write(value);
    }
}

fn disable_nmi() {
        let current = unsafe { Port::<u8>::new(CMOS_ADDR_PORT).read() };

        let mut new_value = current & !RTC_REGISTER_B_MASK;
        new_value |= RTC_NMI_DISABLE_MASK;

        out_cmos_register(CMOS_STATUS_B, new_value);
}

fn shutdown() {
        disable_nmi();
        out_cmos_register(CMOS_STATUS_A, ACPI_OFF);
}
