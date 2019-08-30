extern crate rppal;

use rppal::gpio::{Gpio, Level};
use rppal::spi::{Spi};
use std::{thread, time};

use crate::errors::VSMPError;

pub enum PinNumber {
    RSTPin = 17,
    DCPin = 25,
    CSPin = 8,
    BUSYPin = 24,
    BCMPin = 18
}

impl PinNumber {
    fn value(&self) -> u8 {
        match *self {
            PinNumber::RSTPin => 17,
            PinNumber::DCPin => 25,
            PinNumber::CSPin => 8,
            PinNumber::BUSYPin => 24,
            PinNumber::BCMPin => 18
        }
    }
}

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
        self.gpio.get(PinNumber::BUSYPin.value())?.into_output();
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
    pub fn sleep_ms(&self, ms: u64) {
        let ten_millis = time::Duration::from_millis(ms);
        thread::sleep(ten_millis);
    }
}
