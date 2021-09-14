mod ueberzug;

use crate::display::Displayable;
use crate::errors::VSMPError;

use ueberzug::Ueberzug;

pub enum Terminal {
    Ueberzug,
}

impl Terminal {
    pub fn default(&self) -> Result<Box<dyn Displayable>, VSMPError> {
        match self {
            Terminal::Ueberzug => Ok(Box::new(Ueberzug::default()?)),
        }
    }
}
