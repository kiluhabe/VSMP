extern crate ctrlc;
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
mod vsmp;

use std::path::Path;

use cache::Cache;
use errors::VSMPError;
use vsmp::{VSMPConfig, VSMP};

fn main() -> Result<(), VSMPError> {
    let cache = Cache::default()?;
    cache.init()?;

    let config = VSMPConfig {
        src: Path::new("/home/kiluhabe/codes/VSMP/sample.mkv"),
        cache: &cache.path,
        fps: 24f32,
        interval: 1000,
        height: 100,
        width: 100,
    };
    let mut vsmp = VSMP::default()?;

    ctrlc::set_handler(move || {
        let cache = Cache::default().unwrap();
        cache.purge().unwrap();
    })?;
    vsmp.play(config)?;
    cache.purge()?;
    Ok(())
}
