use rppal::gpio::{Gpio, Level};
use rppal::spi::{Spi};

use crate::pin::PinNumber;
use crate::errors::VSMPError;

pub struct EPDInterface {
    pub spi: Spi,
    pub gpio: Gpio
}

impl EPDInterface {
    pub fn write(&self, pin_number: PinNumber, level: Level) -> Result<(), VSMPError> {
        self.gpio.get(pin_number.value())?.into_output().write(level);
        Ok(())
    }
    pub fn read(&self, pin_number: PinNumber) -> Result<Level, VSMPError> {
        Ok(self.gpio.get(pin_number.value())?.into_input().read())
    }
    pub fn spi_write(&mut self, data: &[u8]) -> Result<(), VSMPError> {
        self.write(PinNumber::CSPin, Level::Low)?;
        self.spi.write(data)?;
        self.write(PinNumber::CSPin, Level::High)?;
        Ok(())
    }
}
