use std::path::Path;
use std::process::Command;

use crate::cache::Cache;
use crate::errors::VSMPError;

pub struct FFmpeg;

impl FFmpeg {
    pub fn capture(&self, src: &Path, dist_dir: &Path, sec: u32) -> Result<Box<Path>, VSMPError> {
        let file_name = format!(
            "{}-{}.jpg",
            src.file_stem().unwrap().to_str().unwrap(),
            sec.to_string()
        );
        let dist = dist_dir.join(&file_name);
        Command::new("ffmpeg")
            .args(&[
                "-i",
                src.to_str().unwrap(),
                "-vframes",
                "1",
                "-an",
                "-ss",
                &sec.to_string(),
                dist.to_str().unwrap(),
            ])
            .output()?;
        Ok(Box::from(dist))
    }
}
