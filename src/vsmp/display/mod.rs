pub mod epd;
mod image_converter;
pub mod terminal;

use crate::vsmp::errors::{InvalidDisplayError, VsmpError};
use std::path::Path;
use async_trait::async_trait;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Display {
    Epd,
    Ueberzug,
}

#[async_trait]
pub trait Displayable {
    async fn display(&mut self, path: &Path, height: u32, width: u32) -> Result<(), VsmpError>;
}

impl std::str::FromStr for Display {
    type Err = VsmpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "epd" => Ok(Display::Epd),
            "ueberzug" => Ok(Display::Ueberzug),
            _ => Err(VsmpError::InvalidDisplay(InvalidDisplayError {})),
        }
    }
}

impl Display {
    pub fn get(&self) -> Result<Box<dyn Displayable + Sync + Send>, VsmpError> {
        match self {
            Display::Epd => Ok(epd::Epd::default()?),
            Display::Ueberzug => Ok(terminal::Terminal::default()?),
        }
    }
}
