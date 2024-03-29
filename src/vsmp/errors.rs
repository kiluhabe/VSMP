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
pub enum VsmpError {
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

impl fmt::Display for VsmpError {
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

impl std::error::Error for VsmpError {}
unsafe impl Send for VsmpError {}
unsafe impl Sync for VsmpError {}

impl From<gpio::Error> for VsmpError {
    fn from(err: gpio::Error) -> VsmpError {
        VsmpError::Gpio(err)
    }
}

impl From<spi::Error> for VsmpError {
    fn from(err: spi::Error) -> VsmpError {
        VsmpError::Spi(err)
    }
}

impl From<ImageSizeError> for VsmpError {
    fn from(err: ImageSizeError) -> VsmpError {
        VsmpError::ImageSize(err)
    }
}

impl From<ImageError> for VsmpError {
    fn from(err: ImageError) -> VsmpError {
        VsmpError::Image(err)
    }
}

impl From<serde_json::Error> for VsmpError {
    fn from(err: serde_json::Error) -> VsmpError {
        VsmpError::SerdeJson(err)
    }
}

impl From<io::Error> for VsmpError {
    fn from(err: io::Error) -> VsmpError {
        VsmpError::IO(err)
    }
}

impl From<string::FromUtf8Error> for VsmpError {
    fn from(err: string::FromUtf8Error) -> VsmpError {
        VsmpError::FromUtf8(err)
    }
}

impl From<CacheDirError> for VsmpError {
    fn from(err: CacheDirError) -> VsmpError {
        VsmpError::CacheDir(err)
    }
}

impl From<ParseFloatError> for VsmpError {
    fn from(err: ParseFloatError) -> VsmpError {
        VsmpError::ParseFloat(err)
    }
}

impl From<ctrlc::Error> for VsmpError {
    fn from(err: ctrlc::Error) -> VsmpError {
        VsmpError::Ctrlc(err)
    }
}

impl From<InvalidDisplayError> for VsmpError {
    fn from(err: InvalidDisplayError) -> VsmpError {
        VsmpError::InvalidDisplay(err)
    }
}
