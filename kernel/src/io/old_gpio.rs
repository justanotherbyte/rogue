use super::{mmio_read, mmio_write};
use crate::bsp::{GPCLR0, GPFSEL0, GPIO_FUNCTION_ALT5, GPIO_MAX_PIN, GPPUPPDN0, GPSET0};

pub fn gpio_call(pin_number: u32, value: u32, base: u32, field_size: u32, field_max: u32) {
    let field_mask: u32 = (1 << field_size) - 1;

    if (pin_number > field_max) || (value > field_mask) {
        panic!("RIP")
    };

    let num_fields: u32 = 32 / field_size;
    let reg: u32 = base + ((pin_number / num_fields) * 4);
    let shift = (pin_number % num_fields) * field_size;

    let mut curval: u32 = mmio_read(reg as *const u32);
    curval &= !(field_mask << shift);
    curval |= value << shift;

    mmio_write(reg as *mut u32, curval);
}

pub fn gpio_set(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPSET0, 1, GPIO_MAX_PIN);
}
pub fn gpio_clear(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPCLR0, 1, GPIO_MAX_PIN);
}
pub fn gpio_pull(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPPUPPDN0, 2, GPIO_MAX_PIN);
}
pub fn gpio_function(pin_number: u32, value: u32) {
    gpio_call(pin_number, value, GPFSEL0, 3, GPIO_MAX_PIN);
}

pub fn gpio_use_as_alt5(pin_number: u32) {
    gpio_pull(pin_number, 0);
    gpio_function(pin_number, GPIO_FUNCTION_ALT5);
}
