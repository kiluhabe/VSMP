use rppal::gpio::{Level};
use image::{GrayImage,Luma};
use std::{thread, time};

use crate::epd_interface::{EPDInterface, PinNumber};
use crate::errors::VSMPError;

pub enum Command {
    PanelSetting,
    PowerSetting,
    PowerOff,
    PowerOffSequenceSetting,
    PowerOn,
    PowerOnMeasure,
    BoosterSoftStart,
    DeepSleep,
    DataStartTransmission1,
    DataStop,
    DisplayRefresh,
    ImageProcess,
    LUTForVCOM,
    LUTBlue,
    LUTWhite,
    LUTGray1,
    LUTGray2,
    LUTRed0,
    LUTRed1,
    LUTRed2,
    LUTRed3,
    LUTXon,
    PLLControl,
    TemperatureSensorCommand,
    TemperatureCalibration,
    TemperatureSensorWrite,
    TemperatureSensorRead,
    VCOMAndDataIntervalSetting,
    LowPowerDetection,
    TCONSetting,
    TCONResolution,
    SPIFlashControl,
    Revision,
    GetStatus,
    AutoMeasurementVCOM,
    ReadVCOMValue,
    VCMDCSetting,
    FlashMode,
}

impl Command {
    fn value(&self) -> u8 {
        match *self {
            Command::PanelSetting                               => 0x00,
            Command::PowerSetting                               => 0x01,
            Command::PowerOff                                => 0x02,
            Command::PowerOffSequenceSetting                  => 0x03,
            Command::PowerOn                                    => 0x04,
            Command::PowerOnMeasure                            => 0x05,
            Command::BoosterSoftStart                          => 0x06,
            Command::DeepSleep                            => 0x07,
            Command::DataStartTransmission1                   => 0x10,
            Command::DataStop                                   => 0x11,
            Command::DisplayRefresh                             => 0x12,
            Command::ImageProcess                              => 0x13,
            Command::LUTForVCOM                                => 0x20,
            Command::LUTBlue                                  => 0x21,
            Command::LUTWhite                                   => 0x22,
            Command::LUTGray1                                  => 0x23,
            Command::LUTGray2                                  => 0x24,
            Command::LUTRed0                                   => 0x25,
            Command::LUTRed1                                   => 0x26,
            Command::LUTRed2                                   => 0x27,
            Command::LUTRed3                                   => 0x28,
            Command::LUTXon                                     => 0x29,
            Command::PLLControl                                 => 0x30,
            Command::TemperatureSensorCommand                  => 0x40,
            Command::TemperatureCalibration                     => 0x41,
            Command::TemperatureSensorWrite                    => 0x42,
            Command::TemperatureSensorRead                     => 0x43,
            Command::VCOMAndDataIntervalSetting              => 0x50,
            Command::LowPowerDetection                         => 0x51,
            Command::TCONSetting                                => 0x60,
            Command::TCONResolution                             => 0x61,
            Command::SPIFlashControl                           => 0x65,
            Command::Revision                                    => 0x70,
            Command::GetStatus                                  => 0x71,
            Command::AutoMeasurementVCOM                       => 0x80,
            Command::ReadVCOMValue                             => 0x81,
            Command::VCMDCSetting                              => 0x82,
            Command::FlashMode                                 => 0xe5,
        }
    }
}

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
        self.interface.init()?;

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
    pub fn get_frame_buffer(&self, image: &GrayImage) -> Vec<u8>{
        let buffer_size: u32 = self.height * self.width / 8;
        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
        unsafe {
            buffer.set_len(buffer_size as usize);
        }
        for _i in 0..buffer_size {
            buffer.push(0x00);
        }

        if image.height() != self.height || image.width() != self.width {
            panic!(VSMPError::ImageSize);
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if image.get_pixel(x, y) == &Luma([0u8]) {
                    let address = ((x + y * self.width) / 8) as usize;
                    buffer[address] |= 0x80 >> (x % 8);
                }
            }
        }
        return buffer
    }
    pub fn display_frame(&mut self, buffer: &[u8]) -> Result<(), VSMPError>{
        self.send_command(Command::DataStartTransmission1)?;
        for i in 0..30720 {
            let mut temp1 = buffer[i];
            let mut j = 0;
            while j < 8 {
                let mut temp2: u8 = if (temp1 & 0x80) > 0 {
                    0x03
                } else {
                    0x00
                };
                temp2 = (temp2 << 4) & 0xFF;
                temp1 = (temp1 << 1) & 0xFF;
                j += 1;
                temp2 |= if (temp1 & 0x80) > 0 {
                    0x03
                } else {
                    0x00
                };
                temp1 = (temp1 << 1) & 0xFF;
                self.send_data(&[temp2])?;
                j += 1
            }
            self.send_command(Command::DisplayRefresh)?;
            self.sleep(100);
            self.wait_until_idle()?;
        }
        Ok(())
    }
}
