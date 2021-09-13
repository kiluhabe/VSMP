mod ffprobe;

use std::path::Path;

use ffprobe::FFprobe;

use crate::errors::VSMPError;

pub trait Analyzable {
    fn duration(&self, src: &Path) -> Result<f32, VSMPError>;
}

pub enum Analyzer {
    FFprobe,
}

impl Analyzer {
    pub fn default(&self) -> Box<dyn Analyzable> {
        match self {
            Analyzer::FFprobe => Box::new(FFprobe::default()),
        }
    }
}
