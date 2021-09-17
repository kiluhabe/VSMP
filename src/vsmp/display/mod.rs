pub mod epd;
mod image_converter;
pub mod terminal;

use crate::vsmp::errors::VSMPError;
use std::path::Path;

pub trait Displayable {
    fn display(
        &mut self,
        path: &Path,
        height: u32,
        width: u32,
        wait_millis: u32,
    ) -> Result<(), VSMPError>;
}
