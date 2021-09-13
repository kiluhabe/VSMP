mod cache;
mod ffmpeg;
mod ffprobe;

use std::path::Path;

use cache::Cache;
use ffmpeg::FFmpeg;
use ffprobe::FFprobe;

use crate::errors::VSMPError;

pub trait Capturable {
    fn capture(&self, src: &Path, dist_dir: &Path, sec: f32) -> Result<Box<Path>, VSMPError>;
}

pub trait Analyzable {
    fn duration(&self, src: &Path) -> Result<f32, VSMPError>;
}

pub struct Capture {
    processor: Box<dyn Capturable>,
    analyzer: Box<dyn Analyzable>,
    cache: Cache,
}

impl Capture {
    pub fn default() -> Result<Self, VSMPError> {
        Ok(Self {
            processor: Box::new(FFmpeg::default()),
            analyzer: Box::new(FFprobe::default()),
            cache: Cache::default()?,
        })
    }
    pub fn capture(&self, src: &Path, fps: f32) -> Result<Vec<Box<Path>>, VSMPError> {
        let key = src.file_stem().unwrap().to_str().unwrap();
        let duration = self.analyzer.duration(src)?;
        let step = 1f32 / fps;

        self.cache.init()?;
        let dist = self.cache.create_dir(key)?;

        let mut sec = 0f32;
        let mut thumbnails = vec![];
        while sec <= duration {
            let thumbnail = self.processor.capture(src, &dist, sec)?;
            thumbnails.push(thumbnail);
            sec += step;
        }
        Ok(thumbnails)
    }
    pub fn clean(&self) -> Result<(), VSMPError> {
        self.cache.purge()?;
        Ok(())
    }
}
