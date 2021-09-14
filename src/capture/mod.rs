mod ffmpeg;

use std::path::Path;

use ffmpeg::FFmpeg;

use crate::errors::VSMPError;

pub trait Capturable {
    fn capture(&self, src: &Path, dist_dir: &Path, sec: f32) -> Result<Box<Path>, VSMPError>;
}

pub enum Capture {
    FFmpeg,
}

impl Capture {
    pub fn default(&self) -> Box<dyn Capturable + Sync + Send> {
        match self {
            Capture::FFmpeg => Box::new(FFmpeg::default()),
        }
    }
}
