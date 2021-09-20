mod ffmpeg;

use crate::vsmp::errors::VsmpError;
use ffmpeg::FFmpeg;
use std::path::Path;
use async_trait::async_trait;

#[allow(dead_code)]
pub enum Capture {
    FFmpeg,
}

#[async_trait]
pub trait Capturable {
    async fn capture(&self, src: &Path, dist_dir: &Path, sec: f32) -> Result<Box<Path>, VsmpError>;
}

impl Capture {
    pub fn default() -> Box<dyn Capturable + Sync + Send> {
        Box::new(FFmpeg::default())
    }
}
