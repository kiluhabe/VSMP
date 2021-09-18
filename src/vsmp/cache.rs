use crate::vsmp::errors::{CacheDirError, VsmpError};
use dirs;
use std::fs;
use std::path::Path;

pub struct Cache {
    pub path: Box<Path>,
}

impl Cache {
    pub fn new(path: Option<&Path>) -> Result<Cache, VsmpError> {
        path.map_or(Self::default(), |p| Ok(Cache { path: Box::from(p) }))
    }
    pub fn default() -> Result<Cache, VsmpError> {
        let path = dirs::cache_dir().ok_or(CacheDirError)?.join("vsmp");
        Ok(Cache {
            path: Box::from(path.as_path()),
        })
    }
    pub fn init(&self) -> Result<(), VsmpError> {
        if self.path.exists() {
            Ok(())
        } else {
            fs::create_dir(self.path.clone())?;
            Ok(())
        }
    }
    pub fn purge(&self) -> Result<(), VsmpError> {
        for entry in self.path.read_dir()? {
            fs::remove_file(entry?.path())?;
        }
        Ok(())
    }
}
