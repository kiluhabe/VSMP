extern crate dirs;
extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod analyzer;
mod cache;
mod capture;
mod display;
mod errors;

use std::path::Path;

use analyzer::Analyzer;
use cache::Cache;
use capture::Capture;
use display::terminal::Terminal;
use errors::VSMPError;

fn main() -> Result<(), VSMPError> {
    let fps = 24f32;
    let src = Path::new("/home/kiluhabe/codes/VSMP/sample.mkv");
    let step = 1f32 / fps;
    let interval = 1000;

    let analyzer = Analyzer::FFprobe.default();
    let cache = Cache::default()?;
    let capture = Capture::FFmpeg.default();
    let mut term = Terminal::Ueberzug.default()?;

    let duration = analyzer.duration(src)?;

    cache.init()?;
    cache.purge()?;

    let mut capture_point = 0f32;
    while capture_point <= duration {
        let thumbnail = capture.capture(src, &cache.path, capture_point)?;
        term.display(&thumbnail, 100, 100, interval as u32)?;
        capture_point += step;
    }
    cache.purge()?;
    Ok(())
}
