use crate::vsmp::analyzer::Analyzable;
use crate::vsmp::errors::VsmpError;
use std::path::Path;
use std::process::Command;

#[derive(Copy, Clone)]
pub struct FFprobe;

impl FFprobe {
    pub fn default() -> Self {
        Self {}
    }
}

impl Analyzable for FFprobe {
    fn duration(&self, src: &Path) -> Result<f32, VsmpError> {
        let output = Command::new("ffprobe")
            .args(&[
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                src.to_str().unwrap(),
            ])
            .output()?;
        let stdout = String::from_utf8(output.stdout)?;
        let duration = stdout[0..stdout.len() - 2].parse::<f32>()?;
        Ok(duration)
    }
}

unsafe impl Sync for FFprobe {}
unsafe impl Send for FFprobe {}
