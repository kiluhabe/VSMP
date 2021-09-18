mod ueberzug;

use crate::vsmp::display::Displayable;
use crate::vsmp::errors::VsmpError;
use ueberzug::Ueberzug;

pub enum Terminal {
    Ueberzug,
}

impl Terminal {
    pub fn default() -> Result<Box<dyn Displayable + Sync + Send>, VsmpError> {
        Ok(Box::new(Ueberzug::default()?))
    }
}
