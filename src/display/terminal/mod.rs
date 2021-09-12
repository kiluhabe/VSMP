mod ueberzug;

use crate::display::Displayable;
use ueberzug::Ueberzug;

pub enum Terminal {
    Ueberzug,
}

impl Terminal {
    pub fn default(&self) -> Box<dyn Displayable> {
        match self {
            Terminal::Ueberzug => Box::new(Ueberzug::default()),
        }
    }
}
