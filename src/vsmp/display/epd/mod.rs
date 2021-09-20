mod command;
mod interface;
mod pin;

use crate::vsmp::display::image_converter::ImageConverter;
use crate::vsmp::display::Displayable;
use crate::vsmp::errors::VsmpError;
use command::Command;
use interface::Interface;
use pin::Pin;
use rppal::gpio::Level;
use std::path::Path;
use std::{thread, time};
use async_trait::async_trait;

pub struct Epd {
    interface: Interface,
    image_converter: ImageConverter,
}

impl Epd {
    pub fn default() -> Result<Box<dyn Displayable + Sync + Send>, VsmpError> {
        let interface = Interface::default()?;
        let image_converter = ImageConverter {};
        Ok(Box::new(Epd {
            interface,
            image_converter,
        }))
    }
    fn send_command(&mut self, command: Command) -> Result<(), VsmpError> {
        self.interface.write(Pin::Dc, Level::Low)?;
        self.interface.spi_write(&[command.value()])?;
        Ok(())
    }
    fn send_data(&mut self, data: &[u8]) -> Result<(), VsmpError> {
        self.interface.write(Pin::Dc, Level::High)?;
        self.interface.spi_write(data)?;
        Ok(())
    }
    fn sleep(&self, ms: u32) {
        thread::sleep(time::Duration::from_millis(ms as u64));
    }
    fn reset(&self) -> Result<(), VsmpError> {
        self.interface.write(Pin::Rst, Level::Low)?;
        self.sleep(200);
        self.interface.write(Pin::Rst, Level::High)?;
        self.sleep(200);
        Ok(())
    }
    fn wait_until_idle(&self) -> Result<(), VsmpError> {
        while self.interface.read(Pin::Busy)? == Level::Low {
            self.sleep(100);
        }
        Ok(())
    }
    pub fn init(&mut self) -> Result<(), VsmpError> {
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

#[async_trait]
impl Displayable for Epd {
    async fn display(&mut self, path: &Path, height: u32, width: u32) -> Result<(), VsmpError> {
        let buffer = self.image_converter.convert(path, height, width)?;
        self.init()?;
        self.sleep(200);
        self.send_command(Command::DataStartTransmission1)?;
        for i in buffer {
            self.send_data(&[i])?;
        }
        self.send_command(Command::DisplayRefresh)?;
        self.sleep(100);
        self.wait_until_idle()?;
        Ok(())
    }
}

unsafe impl Sync for Epd {}
unsafe impl Send for Epd {}
