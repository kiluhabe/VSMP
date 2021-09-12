use image::ImageError;
use rppal::{gpio, spi};
use serde_json;

use std::error;
use std::fmt;
use std::io;
use std::string;

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

#[derive(Debug, Clone)]
pub struct SerdeJSONCastError;

impl fmt::Display for SerdeJSONCastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to casting.")
    }
}

impl error::Error for SerdeJSONCastError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum VSMPError {
    Gpio(gpio::Error),
    Spi(spi::Error),
    ImageSize(ImageSizeError),
    Image(ImageError),
    SerdeJson(serde_json::Error),
    IO(io::Error),
    FromUtf8(string::FromUtf8Error),
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

impl From<serde_json::Error> for VSMPError {
    fn from(err: serde_json::Error) -> VSMPError {
        VSMPError::SerdeJson(err)
    }
}

impl From<io::Error> for VSMPError {
    fn from(err: io::Error) -> VSMPError {
        VSMPError::IO(err)
    }
}

impl From<string::FromUtf8Error> for VSMPError {
    fn from(err: string::FromUtf8Error) -> VSMPError {
        VSMPError::FromUtf8(err)
    }
}
