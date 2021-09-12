extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod display;
mod errors;
mod image_converter;

use std::path::Path;

use display::terminal::Terminal;
use errors::VSMPError;

fn main() -> Result<(), VSMPError> {
    let path = Path::new("/home/kiluhabe/codes/VSMP/sample.png");

    let mut term = Terminal::Ueberzug.default();

    term.display(&path, 384, 640)?;

    Ok(())
}
