extern crate ctrlc;
extern crate dirs;
extern crate image;
extern crate rppal;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod vsmp;

use std::path::Path;
use vsmp::cache::Cache;
use vsmp::errors::VSMPError;
use vsmp::{Config, VSMP};

fn main() -> Result<(), VSMPError> {
    let cache = Cache::default()?;
    cache.init()?;

    let config = Config {
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
