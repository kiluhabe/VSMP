mod ffmpeg;

use crate::vsmp::errors::VsmpError;
use ffmpeg::FFmpeg;
use std::path::Path;

pub enum Capture {
    FFmpeg,
}

pub trait Capturable {
    fn capture(&self, src: &Path, dist_dir: &Path, sec: f32) -> Result<Box<Path>, VsmpError>;
}

impl Capture {
    pub fn default() -> Box<dyn Capturable + Sync + Send> {
        Box::new(FFmpeg::default())
    }
}
