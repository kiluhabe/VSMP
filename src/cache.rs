use std::fs;
use std::path::Path;

use dirs;

use crate::errors::{CacheDirError, VSMPError};

pub struct Cache {
    pub path: Box<Path>,
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
    pub fn purge(&self) -> Result<(), VSMPError> {
        for entry in self.path.read_dir()? {
            fs::remove_file(entry?.path())?;
        }
        Ok(())
    }
}
