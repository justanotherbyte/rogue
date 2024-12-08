pub mod gpio;
pub mod uart;

pub fn mmio_write(reg: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(reg as *mut u32, val);
    }
}

pub fn mmio_read(reg: u32) -> u32 {
    let val: u32;
    unsafe {
        val = core::ptr::read_volatile(reg as *const u32);
    }
    val
}
