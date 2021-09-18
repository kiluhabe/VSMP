mod ffprobe;

use crate::vsmp::errors::VsmpError;
use ffprobe::FFprobe;
use std::path::Path;

#[allow(dead_code)]
pub enum Analyzer {
    FFprobe,
}

pub trait Analyzable {
    fn duration(&self, src: &Path) -> Result<f32, VsmpError>;
}

impl Analyzer {
    pub fn default() -> Box<dyn Analyzable + Sync + Send> {
        Box::new(FFprobe::default())
    }
}
