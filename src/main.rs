extern crate dirs;
extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod capture;
mod display;
mod errors;

use std::path::Path;

use capture::Capture;
use display::terminal::Terminal;
use errors::VSMPError;

fn main() -> Result<(), VSMPError> {
    let speed = 0.5;
    let fps = 24f32;
    let src = Path::new("/home/kiluhabe/codes/VSMP/sample.mkv");
    let interval = 1f32 / fps / speed;

    let capture = Capture::default()?;
    let mut term = Terminal::Ueberzug.default();

    capture.clean()?;
    let thumbnails = capture.capture(src, fps)?;

    for thumbnail in thumbnails {
        term.display(&thumbnail, 100, 100, interval as u32)?;
    }

    capture.clean()?;

    Ok(())
}
