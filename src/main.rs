extern crate image;
extern crate rppal;

mod converter;
mod display;
mod errors;

use std::path::Path;

use converter::convert;
use display::epd::EPD;
use display::Displayable;
use errors::VSMPError;

fn main() -> Result<(), VSMPError> {
    let mut epd = EPD::default(384, 640)?;
    let path = Path::new("/tmp/vsmp/images/sample.png");
    let buffer = convert(path, epd.width, epd.height)?;

    epd.display(&buffer)?;
    println!("{}", "done.");

    Ok(())
}
