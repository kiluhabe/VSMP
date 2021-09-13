use std::fs;
use std::path::Path;

use dirs;

use crate::errors::{CacheDirError, VSMPError};

pub struct Cache {
    path: Box<Path>,
}

impl Cache {
    pub fn default() -> Result<Cache, VSMPError> {
        let path = dirs::cache_dir().ok_or(CacheDirError)?.join("vsmp");
        Ok(Cache {
            path: Box::from(path.as_path()),
        })
    }
    pub fn init(&self) -> Result<(), VSMPError> {
        if self.path.exists() {
            Ok(())
        } else {
            fs::create_dir(self.path.clone())?;
            Ok(())
        }
    }
    pub fn create_dir(&self, key: &str) -> Result<Box<Path>, VSMPError> {
        let cache_dir = self.path.join(key);
        fs::create_dir(cache_dir.clone())?;
        Ok(Box::from(cache_dir.clone()))
    }
    pub fn purge(&self, key: &str) -> Result<(), VSMPError> {
        let cache_dir = self.path.join(key);
        for entry in cache_dir.read_dir()? {
            fs::remove_file(entry?.path())?
        }
        Ok(())
    }
    pub fn delete_dir(&self, key: &str) -> Result<(), VSMPError> {
        let cache_dir = self.path.join(key);
        fs::remove_dir_all(cache_dir)?;
        Ok(())
    }
}
