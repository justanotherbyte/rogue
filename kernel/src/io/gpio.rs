use super::{mmio_read, mmio_write};
use crate::bsp::{GPCLR0, GPFSEL0, GPIO_FUNCTION_ALT5, GPIO_MAX_PIN, GPPUPPDN0, GPSET0};

#[derive(Debug)]
pub enum Error {
    InvalidGPIOPin,
    ValueLargerThanFieldMask,
}

pub struct Pin {
    number: u8,
}
impl Pin {
    fn gpio_call(&self, value: u32, base: u32, field_size: u32) -> Result<(), Error> {
        let field_mask: u32 = (1 << field_size) - 1;

        if value > field_mask {
            return Err(Error::ValueLargerThanFieldMask);
        };

        let pin_number = self.number as u32;

        let num_fields: u32 = 32 / field_size;
        let reg: u32 = base + ((pin_number / num_fields) * 4);
        let shift = (pin_number % num_fields) * field_size;

        let mut curval: u32 = mmio_read(reg);
        curval &= !(field_mask << shift);
        curval |= value << shift;

        mmio_write(reg, curval);

        Ok(())
    }
    pub fn new(number: u8) -> Result<Self, Error> {
        if number > GPIO_MAX_PIN {
            Err(Error::InvalidGPIOPin)
        } else {
            Ok(Self { number })
        }
    }
    pub fn set(&self, value: u32) -> Result<(), Error> {
        self.gpio_call(value, GPSET0, 1)
    }
    pub fn clear(&self, value: u32) -> Result<(), Error> {
        self.gpio_call(value, GPCLR0, 1)
    }
    pub fn pull(&self, value: u32) -> Result<(), Error> {
        self.gpio_call(value, GPPUPPDN0, 2)
    }
    pub fn function(&self, value: u32) -> Result<(), Error> {
        self.gpio_call(value, GPFSEL0, 3)
    }
    pub fn use_as_alt5(&self) -> Result<(), Error> {
        self.pull(0)?;
        self.function(GPIO_FUNCTION_ALT5)?;

        Ok(())
    }
}
