extern crate dirs;
extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod cache;
mod display;
mod errors;
mod ffmpeg;
mod image_converter;

use std::path::Path;

use cache::Cache;
use display::terminal::Terminal;
use errors::VSMPError;
use ffmpeg::FFmpeg;

fn main() -> Result<(), VSMPError> {
    let src = Path::new("/home/kiluhabe/codes/VSMP/sample.mkv");
    let key = src.file_stem().unwrap().to_str().unwrap();

    let mut term = Terminal::Ueberzug.default();
    let ffmpeg = FFmpeg {};
    let cache = Cache::default()?;

    cache.init()?;
    let dist = cache.create_dir(key)?;

    for duration in 1..100 {
        let thumbnail = ffmpeg.capture(src, &dist, duration)?;
        term.display(&thumbnail, 100, 100)?;
        cache.purge(key)?;
    }
    cache.delete_dir(key)?;

    Ok(())
}
