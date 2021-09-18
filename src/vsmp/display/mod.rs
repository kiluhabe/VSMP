pub mod epd;
mod image_converter;
pub mod terminal;

use crate::vsmp::errors::{InvalidDisplayError, VSMPError};
use std::path::Path;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Display {
    EPD,
    Ueberzug,
}

pub trait Displayable {
    fn display(&mut self, path: &Path, height: u32, width: u32) -> Result<(), VSMPError>;
}

impl std::str::FromStr for Display {
    type Err = VSMPError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "epd" => Ok(Display::EPD),
            "ueberzug" => Ok(Display::Ueberzug),
            _ => Err(VSMPError::InvalidDisplay(InvalidDisplayError {})),
        }
    }
}

impl Display {
    pub fn get(&self) -> Result<Box<dyn Displayable + Sync + Send>, VSMPError> {
        match self {
            Display::EPD => Ok(epd::EPD::default()?),
            Display::Ueberzug => Ok(terminal::Terminal::default()?),
        }
    }
}
