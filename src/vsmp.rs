use std::path::Path;

use crate::analyzer::{Analyzable, Analyzer};
use crate::capture::{Capturable, Capture};
use crate::display::terminal::Terminal;
use crate::display::Displayable;
use crate::errors::VSMPError;

pub struct VSMP {
    analyzer: Box<dyn Analyzable + Sync + Send>,
    capture: Box<dyn Capturable + Sync + Send>,
    display: Box<dyn Displayable + Sync + Send>,
}

pub struct VSMPConfig<'a> {
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
    pub fn play(&mut self, config: VSMPConfig) -> Result<(), VSMPError> {
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

impl<'a> VSMPConfig<'a> {
    pub fn step(&self) -> f32 {
        1f32 / self.fps
    }
}
