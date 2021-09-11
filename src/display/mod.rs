pub mod epd;

use crate::errors::VSMPError;

pub trait Displayable {
    fn display(&mut self, buffer: &[u8]) -> Result<(), VSMPError>;
}
