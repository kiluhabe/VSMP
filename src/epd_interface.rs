use rppal::gpio::{Gpio, Level};
use rppal::spi::{Spi};

use crate::pin::PinNumber;
use crate::errors::VSMPError;

pub struct EPDInterface {
    pub spi: Spi,
    pub gpio: Gpio
}

impl EPDInterface {
    pub fn init(&self) -> Result<(), VSMPError> {
        self.gpio.get(PinNumber::BCMPin.value())?.into_output();
        self.gpio.get(PinNumber::RSTPin.value())?.into_output();
        self.gpio.get(PinNumber::DCPin.value())?.into_output();
        self.gpio.get(PinNumber::CSPin.value())?.into_output();
        self.gpio.get(PinNumber::BUSYPin.value())?.into_input();
        Ok(())
    }
    pub fn write(&self, pin_number: PinNumber, level: Level) -> Result<(), VSMPError> {
        self.gpio.get(pin_number.value())?.into_output().write(level);
        Ok(())
    }
    pub fn read(&self, pin_number: PinNumber) -> Result<Level, VSMPError> {
        Ok(self.gpio.get(pin_number.value())?.into_input().read())
    }
    pub fn spi_write(&mut self, data: &[u8]) -> Result<(), VSMPError> {
        self.spi.write(data)?;
        Ok(())
    }
}
