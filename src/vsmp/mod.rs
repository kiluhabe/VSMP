mod analyzer;
pub mod cache;
mod capture;
pub mod cli;
mod display;
pub mod errors;

use analyzer::{Analyzable, Analyzer};
use cache::Cache;
use capture::{Capturable, Capture};
use display::terminal::Terminal;
use display::{Display, Displayable};
use errors::VSMPError;
use std::path::Path;
use std::time::Duration;

pub struct VSMP {
    analyzer: Box<dyn Analyzable + Sync + Send>,
    cache: Cache,
    capture: Box<dyn Capturable + Sync + Send>,
    display: Box<dyn Displayable + Sync + Send>,
}

#[derive(Copy, Clone)]
pub struct Config<'a> {
    pub height: u32,
    pub width: u32,
    pub fps: f32,
    pub interval: u32,
    pub src: &'a Path,
    pub display: Display,
    pub cache: Option<&'a Path>,
}

impl VSMP {
    pub fn new(config: Config) -> Result<Self, VSMPError> {
        Ok(VSMP {
            analyzer: Analyzer::default(),
            cache: Cache::new(config.cache)?,
            capture: Capture::default(),
            display: config.display.get()?,
        })
    }
    pub fn default() -> Result<Self, VSMPError> {
        Ok(VSMP {
            analyzer: Analyzer::default(),
            cache: Cache::default()?,
            capture: Capture::default(),
            display: Terminal::default()?,
        })
    }
    pub fn play(&mut self, config: Config) -> Result<(), VSMPError> {
        let duration = self.analyzer.duration(&config.src)?;
        let mut capture_point = 0f32;
        self.cache.init()?;
        while capture_point <= duration {
            let thumbnail = self
                .capture
                .capture(&config.src, &self.cache.path, capture_point)?;
            self.display.display(&thumbnail, 100, 100)?;
            std::thread::sleep(Duration::from_millis(config.interval as u64));
            self.cache.purge()?;
            capture_point += config.step();
        }
        Ok(())
    }
    pub fn cleanup(&self) -> Result<(), VSMPError> {
        self.cache.purge()
    }
}

impl<'a> Config<'a> {
    pub fn step(&self) -> f32 {
        1f32 / self.fps
    }
}

unsafe impl Sync for VSMP {}
unsafe impl Send for VSMP {}
