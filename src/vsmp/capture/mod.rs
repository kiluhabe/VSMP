mod ffmpeg;

use crate::vsmp::errors::VSMPError;
use ffmpeg::FFmpeg;
use std::path::Path;

pub enum Capture {
    FFmpeg,
}

pub trait Capturable {
    fn capture(&self, src: &Path, dist_dir: &Path, sec: f32) -> Result<Box<Path>, VSMPError>;
}

impl Capture {
    pub fn default(&self) -> Box<dyn Capturable + Sync + Send> {
        match self {
            Capture::FFmpeg => Box::new(FFmpeg::default()),
        }
    }
}
