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
use errors::VsmpError;
use std::path::Path;
use std::time::Duration;

pub struct Vsmp {
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

impl Vsmp {
    pub fn new(config: Config) -> Result<Self, VsmpError> {
        Ok(Vsmp {
            analyzer: Analyzer::default(),
            cache: Cache::new(config.cache)?,
            capture: Capture::default(),
            display: config.display.get()?,
        })
    }
    #[allow(dead_code)]
    pub fn default() -> Result<Self, VsmpError> {
        Ok(Vsmp {
            analyzer: Analyzer::default(),
            cache: Cache::default()?,
            capture: Capture::default(),
            display: Terminal::default()?,
        })
    }
    pub fn play(&mut self, config: Config) -> Result<(), VsmpError> {
        let duration = self.analyzer.duration(config.src)?;
        let mut capture_point = 0f32;
        self.cache.init()?;
        while capture_point <= duration {
            let thumbnail = self
                .capture
                .capture(config.src, &self.cache.path, capture_point)?;
            self.display.display(&thumbnail, 100, 100)?;
            std::thread::sleep(Duration::from_millis(config.interval as u64));
            self.cache.purge()?;
            capture_point += config.step();
        }
        Ok(())
    }
    pub fn cleanup(&self) -> Result<(), VsmpError> {
        self.cache.purge()
    }
}

impl<'a> Config<'a> {
    pub fn step(&self) -> f32 {
        1f32 / self.fps
    }
}

unsafe impl Sync for Vsmp {}
unsafe impl Send for Vsmp {}
