use image::ImageError;

use ctrlc;
use rppal::{gpio, spi};
use serde_json;
use std::error;
use std::fmt;
use std::io;
use std::num::ParseFloatError;
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
pub struct CacheDirError;

impl fmt::Display for CacheDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "home dir could not be found.")
    }
}

impl error::Error for CacheDirError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDisplayError;

impl fmt::Display for InvalidDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid display.")
    }
}

impl error::Error for InvalidDisplayError {
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
    CacheDir(CacheDirError),
    ParseFloat(ParseFloatError),
    Ctrlc(ctrlc::Error),
    InvalidDisplay(InvalidDisplayError),
}

impl fmt::Display for VSMPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Gpio(e) => e.fmt(f),
            Self::Spi(e) => e.fmt(f),
            Self::ImageSize(e) => e.fmt(f),
            Self::Image(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
            Self::IO(e) => e.fmt(f),
            Self::FromUtf8(e) => e.fmt(f),
            Self::CacheDir(e) => e.fmt(f),
            Self::ParseFloat(e) => e.fmt(f),
            Self::Ctrlc(e) => e.fmt(f),
            Self::InvalidDisplay(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for VSMPError {}
unsafe impl Send for VSMPError {}
unsafe impl Sync for VSMPError {}

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

impl From<CacheDirError> for VSMPError {
    fn from(err: CacheDirError) -> VSMPError {
        VSMPError::CacheDir(err)
    }
}

impl From<ParseFloatError> for VSMPError {
    fn from(err: ParseFloatError) -> VSMPError {
        VSMPError::ParseFloat(err)
    }
}

impl From<ctrlc::Error> for VSMPError {
    fn from(err: ctrlc::Error) -> VSMPError {
        VSMPError::Ctrlc(err)
    }
}

impl From<InvalidDisplayError> for VSMPError {
    fn from(err: InvalidDisplayError) -> VSMPError {
        VSMPError::InvalidDisplay(err)
    }
}
