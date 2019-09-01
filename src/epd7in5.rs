use rppal::gpio::{Level};
use std::{thread, time};

use crate::epd_interface::EPDInterface;
use crate::pin::PinNumber;
use crate::errors::VSMPError;
use crate::command::Command;

pub struct EPD {
    pub height: u32,
    pub width: u32,
    pub interface: EPDInterface
}

impl EPD {
    fn send_command(&mut self, command: Command) -> Result<(), VSMPError> {
        self.interface.write(PinNumber::DCPin, Level::Low)?;
        self.interface.spi_write(&[command.value()])?;
        Ok(())
    }
    fn send_data(&mut self, data: &[u8]) -> Result<(), VSMPError>{
        self.interface.write(PinNumber::DCPin, Level::High)?;
        self.interface.spi_write(data)?;
        Ok(())
    }
    fn sleep(&self, ms: u64) {
        thread::sleep(time::Duration::from_millis(ms));
    }
    fn reset(&self) -> Result<(), VSMPError>{
        self.interface.write(PinNumber::RSTPin, Level::Low)?;
        self.sleep(200);
        self.interface.write(PinNumber::RSTPin, Level::High)?;
        self.sleep(200);
        Ok(())
    }
    fn wait_until_idle(&self) -> Result<(), VSMPError>{
        while self.interface.read(PinNumber::BUSYPin)? == Level::Low {
            self.sleep(100);
        }
        Ok(())
    }
    pub fn init(&mut self) -> Result<(), VSMPError>{
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
    pub fn display_frame(&mut self, buffer: &[u8]) -> Result<(), VSMPError>{
        self.send_command(Command::DataStartTransmission1)?;
        for i in 0..(self.width / 8 * self.height) {
            for _ in 1..4 {
                let mut temp1 = buffer[i as usize];
                let mut temp2 = if (temp1 & 0x80) > 0 {
                    0x03
                } else {
                    0x00
                };
                temp2 <<= 4;
                temp1 <<=1;
                temp2 |= if (temp1 & 0x80) > 0 {
                    0x03
                } else {
                    0x00
                };
                self.send_data(&[temp2])?;
            }
            self.send_command(Command::DisplayRefresh)?;
            self.sleep(100);
            self.wait_until_idle()?;
        }
        Ok(())
    }
}
