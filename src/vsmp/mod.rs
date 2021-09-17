mod analyzer;
pub mod cache;
mod capture;
mod display;
pub mod errors;

use std::path::Path;

use analyzer::{Analyzable, Analyzer};
use capture::{Capturable, Capture};
use display::terminal::Terminal;
use display::Displayable;
use errors::VSMPError;

pub struct VSMP {
    analyzer: Box<dyn Analyzable + Sync + Send>,
    capture: Box<dyn Capturable + Sync + Send>,
    display: Box<dyn Displayable + Sync + Send>,
}

pub struct Config<'a> {
    pub height: u32,
    pub width: u32,
    pub fps: f32,
    pub interval: u32,
    pub src: &'a Path,
    pub cache: &'a Path,
}

impl VSMP {
    pub fn default() -> Result<Self, VSMPError> {
        Ok(VSMP {
            analyzer: Analyzer::FFprobe.default(),
            capture: Capture::FFmpeg.default(),
            display: Terminal::Ueberzug.default()?,
        })
    }
    pub fn play(&mut self, config: Config) -> Result<(), VSMPError> {
        let duration = self.analyzer.duration(&config.src)?;
        let mut capture_point = 0f32;
        while capture_point <= duration {
            let thumbnail = self
                .capture
                .capture(&config.src, config.cache, capture_point)?;
            self.display
                .display(&thumbnail, 100, 100, config.interval as u32)?;
            capture_point += config.step();
        }
        Ok(())
    }
}

impl<'a> Config<'a> {
    pub fn step(&self) -> f32 {
        1f32 / self.fps
    }
}
