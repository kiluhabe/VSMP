mod command;
mod interface;
mod pin;

use rppal::gpio::Level;
use std::{thread, time};

use crate::display::Displayable;
use crate::errors::VSMPError;
use command::Command;
use interface::Interface;
use pin::PinNumber;

pub struct EPD {
    pub height: u32,
    pub width: u32,
    pub interface: Interface,
}

impl EPD {
    pub fn default(height: u32, width: u32) -> Result<Self, VSMPError> {
        let interface = Interface::default()?;
        Ok(EPD {
            height: height,
            width: width,
            interface: interface,
        })
    }
    fn send_command(&mut self, command: Command) -> Result<(), VSMPError> {
        self.interface.write(PinNumber::DCPin, Level::Low)?;
        self.interface.spi_write(&[command.value()])?;
        Ok(())
    }
    fn send_data(&mut self, data: &[u8]) -> Result<(), VSMPError> {
        self.interface.write(PinNumber::DCPin, Level::High)?;
        self.interface.spi_write(data)?;
        Ok(())
    }
    fn sleep(&self, ms: u64) {
        thread::sleep(time::Duration::from_millis(ms));
    }
    fn reset(&self) -> Result<(), VSMPError> {
        self.interface.write(PinNumber::RSTPin, Level::Low)?;
        self.sleep(200);
        self.interface.write(PinNumber::RSTPin, Level::High)?;
        self.sleep(200);
        Ok(())
    }
    fn wait_until_idle(&self) -> Result<(), VSMPError> {
        while self.interface.read(PinNumber::BUSYPin)? == Level::Low {
            self.sleep(100);
        }
        Ok(())
    }
    pub fn init(&mut self) -> Result<(), VSMPError> {
        self.reset()?;

        self.send_command(Command::PowerSetting)?;
        self.send_data(&[0x37])?;
        self.send_data(&[0x00])?;

        self.send_command(Command::PanelSetting)?;
        self.send_data(&[0xCF])?;
        self.send_data(&[0x08])?;

        self.send_command(Command::BoosterSoftStart)?;
        self.send_data(&[0xc7])?;
        self.send_data(&[0xcc])?;
        self.send_data(&[0x28])?;

        self.send_command(Command::PowerOn)?;
        self.wait_until_idle()?;

        self.send_command(Command::PLLControl)?;
        self.send_data(&[0x3c])?;

        self.send_command(Command::TemperatureCalibration)?;
        self.send_data(&[0x00])?;

        self.send_command(Command::VCOMAndDataIntervalSetting)?;
        self.send_data(&[0x77])?;

        self.send_command(Command::TCONSetting)?;
        self.send_data(&[0x22])?;

        self.send_command(Command::TCONResolution)?;
        self.send_data(&[0x02])?;
        self.send_data(&[0x80])?;
        self.send_data(&[0x01])?;
        self.send_data(&[0x80])?;

        self.send_command(Command::VCMDCSetting)?;
        self.send_data(&[0x1E])?;

        self.send_command(Command::FlashMode)?;
        self.send_data(&[0x03])?;

        Ok(())
    }
}

impl Displayable for EPD {
    fn display(&mut self, buffer: &[u8]) -> Result<(), VSMPError> {
        self.init()?;
        thread::sleep(time::Duration::from_millis(200));
        self.send_command(Command::DataStartTransmission1)?;
        for i in buffer {
            self.send_data(&[*i])?;
        }
        self.send_command(Command::DisplayRefresh)?;
        self.sleep(100);
        self.wait_until_idle()?;
        Ok(())
    }
}
