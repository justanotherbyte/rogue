use super::gpio::gpio_use_as_alt5;
use super::{mmio_read, mmio_write};
use crate::bsp::{
    AUX_ENABLES, AUX_MU_BAUD_REG, AUX_MU_CNTL_REG, AUX_MU_IER_REG, AUX_MU_IIR_REG, AUX_MU_IO_REG,
    AUX_MU_LCR_REG, AUX_MU_LSR_REG, AUX_MU_MCR_REG, AUX_UART_CLOCK,
};

fn aux_mu_baud(baud: u32) -> u32 {
    (AUX_UART_CLOCK / (baud * 8)) - 1
}

fn is_write_ready() -> bool {
    let val = mmio_read(AUX_MU_LSR_REG as *const u32) & 0x20;
    !matches!(val, 0)
}

pub fn init() {
    mmio_write(AUX_ENABLES as *mut u32, 1); //enable UART1
    mmio_write(AUX_MU_IER_REG as *mut u32, 0);
    mmio_write(AUX_MU_CNTL_REG as *mut u32, 0);
    mmio_write(AUX_MU_LCR_REG as *mut u32, 3); //8 bits
    mmio_write(AUX_MU_MCR_REG as *mut u32, 0);
    mmio_write(AUX_MU_IER_REG as *mut u32, 0);
    mmio_write(AUX_MU_IIR_REG as *mut u32, 0xC6); //disable interrupts
    mmio_write(AUX_MU_BAUD_REG as *mut u32, aux_mu_baud(115200));
    gpio_use_as_alt5(14);
    gpio_use_as_alt5(15);
    mmio_write(AUX_MU_CNTL_REG as *mut u32, 3);
}

fn write_char(c: char) {
    while !(is_write_ready()) {}
    mmio_write(AUX_MU_IO_REG as *mut u32, c as u32);
}

pub fn write(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}

pub fn writeln(s: &str) {
    write(s);
    write("\r");
}
