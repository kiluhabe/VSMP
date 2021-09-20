use crate::vsmp::Config;
use clap;
use clap::{AppSettings, Clap};
use std::path::Path;

#[derive(Clap, Debug, Clone)]
#[clap(version = "1.0", author = "kiluhabe")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(index = 1)]
    src: String,
    #[clap(short, long)]
    height: u32,
    #[clap(short, long)]
    width: u32,
    #[clap(short, long)]
    fps: f32,
    #[clap(short, long)]
    interval: u32,
    #[clap(short, long)]
    cache: Option<String>,
    #[clap(short, long)]
    debug: Option<i32>,
}

impl Options {
    pub fn to_config(&self) -> Config {
        Config {
            src: Path::new(&self.src),
            width: self.width,
            height: self.height,
            fps: self.fps,
            interval: self.interval,
            cache: self.cache.as_ref().map(|path| Path::new(path)),
            debug: self.debug
        }
    }
}
