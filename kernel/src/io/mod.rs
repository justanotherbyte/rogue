pub mod gpio;
pub mod uart;

pub fn mmio_write(reg: *mut u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(reg, val);
    }
}

pub fn mmio_read(reg: *const u32) -> u32 {
    let val: u32;
    unsafe {
        val = core::ptr::read_volatile(reg);
    }
    val
}
