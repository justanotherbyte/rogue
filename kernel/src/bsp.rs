const PERIPHERAL_BASE: u32 = 0xFE000000;
// GPIO
pub const GPFSEL0: u32 = PERIPHERAL_BASE + 0x200000;
pub const GPSET0: u32 = PERIPHERAL_BASE + 0x20001C;
pub const GPCLR0: u32 = PERIPHERAL_BASE + 0x200028;
pub const GPPUPPDN0: u32 = PERIPHERAL_BASE + 0x2000E4;

pub const GPIO_MAX_PIN: u32 = 53;
pub const GPIO_FUNCTION_ALT5: u32 = 2;

// AUX
pub const AUX_BASE: u32 = PERIPHERAL_BASE + 0x215000;
pub const AUX_ENABLES: u32 = AUX_BASE + 4;
pub const AUX_MU_IO_REG: u32 = AUX_BASE + 64;
pub const AUX_MU_IER_REG: u32 = AUX_BASE + 68;
pub const AUX_MU_IIR_REG: u32 = AUX_BASE + 72;
pub const AUX_MU_LCR_REG: u32 = AUX_BASE + 76;
pub const AUX_MU_MCR_REG: u32 = AUX_BASE + 80;
pub const AUX_MU_LSR_REG: u32 = AUX_BASE + 84;
pub const AUX_MU_CNTL_REG: u32 = AUX_BASE + 96;
pub const AUX_MU_BAUD_REG: u32 = AUX_BASE + 104;
pub const AUX_UART_CLOCK: u32 = 500000000;
pub const UART_MAX_QUEUE: u32 = 16 * 1024;