pub mod epd;

use std::path::Path;

use crate::errors::VSMPError;

pub trait Displayable {
    fn display(&mut self, path: &Path, height: u32, width: u32) -> Result<(), VSMPError>;
}
