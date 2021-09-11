extern crate image;
extern crate rppal;

mod display;
mod errors;
mod image_converter;

use std::path::Path;

use display::epd::EPD;
use display::Displayable;
use errors::VSMPError;

fn main() -> Result<(), VSMPError> {
    let mut epd = EPD::default()?;
    let path = Path::new("/tmp/vsmp/images/sample.png");

    epd.display(&path, 384, 640)?;
    println!("{}", "done.");

    Ok(())
}
