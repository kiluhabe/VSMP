use rppal::{gpio, spi};
use std::fmt;
use std::error;
use image::ImageError;

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
    ImageSize(ImageSizeError),
    Image(ImageError)
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

impl From<ImageError> for VSMPError {
    fn from(err: ImageError) -> VSMPError {
        VSMPError::Image(err)
    }
}
