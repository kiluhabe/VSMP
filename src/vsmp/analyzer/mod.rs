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
    pub fn default(&self) -> Box<dyn Analyzable + Sync + Send> {
        match self {
            Analyzer::FFprobe => Box::new(FFprobe::default()),
        }
    }
}
