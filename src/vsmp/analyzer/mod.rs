mod ffprobe;

use crate::vsmp::errors::VSMPError;
use ffprobe::FFprobe;
use std::path::Path;

pub enum Analyzer {
    FFprobe,
}

pub trait Analyzable {
    fn duration(&self, src: &Path) -> Result<f32, VSMPError>;
}

impl Analyzer {
    pub fn default() -> Box<dyn Analyzable + Sync + Send> {
        Box::new(FFprobe::default())
    }
}
