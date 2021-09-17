mod ueberzug;

use crate::vsmp::display::Displayable;
use crate::vsmp::errors::VSMPError;
use ueberzug::Ueberzug;

pub enum Terminal {
    Ueberzug,
}

impl Terminal {
    pub fn default(&self) -> Result<Box<dyn Displayable + Sync + Send>, VSMPError> {
        match self {
            Terminal::Ueberzug => Ok(Box::new(Ueberzug::default()?)),
        }
    }
}
