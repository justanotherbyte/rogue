#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

pub mod bsp;
pub mod io;

use io::uart;

global_asm!(include_str!("boot.s"));

#[no_mangle]
fn _start_rust() -> ! {
    uart::init();
    loop {
        uart::writeln("Hello World! From Rogue! new gpio test");
    }
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
