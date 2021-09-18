mod ueberzug;

use crate::vsmp::display::Displayable;
use crate::vsmp::errors::VSMPError;
use ueberzug::Ueberzug;

pub enum Terminal {
    Ueberzug,
}

impl Terminal {
    pub fn default() -> Result<Box<dyn Displayable + Sync + Send>, VSMPError> {
        Ok(Box::new(Ueberzug::default()?))
    }
}
