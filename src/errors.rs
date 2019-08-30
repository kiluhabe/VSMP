extern crate rppal;

use rppal::{gpio, spi};
use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct ImageSizeError;

impl fmt::Display for ImageSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size image")
    }
}

impl error::Error for ImageSizeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum VSMPError {
    Gpio(gpio::Error),
    Spi(spi::Error),
    ImageSize(ImageSizeError)

}

impl From<gpio::Error> for VSMPError {
    fn from(err: gpio::Error) -> VSMPError {
        VSMPError::Gpio(err)
    }
}

impl From<spi::Error> for VSMPError {
    fn from(err: spi::Error) -> VSMPError {
        VSMPError::Spi(err)
    }
}

impl From<ImageSizeError> for VSMPError {
    fn from(err: ImageSizeError) -> VSMPError {
        VSMPError::ImageSize(err)
    }
}
