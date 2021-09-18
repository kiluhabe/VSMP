use rppal::gpio::{Gpio, Level};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use super::pin::PinNumber;
use crate::vsmp::errors::VsmpError;

pub struct Interface {
    pub spi: Spi,
    pub gpio: Gpio,
}

impl Interface {
    pub fn default() -> Result<Self, VsmpError> {
        let gpio = Gpio::new()?;
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2000000, Mode::Mode0)?;
        Ok(Self {
            spi: spi,
            gpio: gpio,
        })
    }
    pub fn write(&self, pin_number: PinNumber, level: Level) -> Result<(), VsmpError> {
        self.gpio
            .get(pin_number.value())?
            .into_output()
            .write(level);
        Ok(())
    }
    pub fn read(&self, pin_number: PinNumber) -> Result<Level, VsmpError> {
        Ok(self.gpio.get(pin_number.value())?.into_input().read())
    }
    pub fn spi_write(&mut self, data: &[u8]) -> Result<(), VsmpError> {
        self.write(PinNumber::CSPin, Level::Low)?;
        self.spi.write(data)?;
        self.write(PinNumber::CSPin, Level::High)?;
        Ok(())
    }
}
