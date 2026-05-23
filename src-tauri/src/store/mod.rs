pub mod secret;

use std::sync::Mutex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct FileScopeManager {
    allowed_folders: Mutex<HashSet<PathBuf>>,
}

impl FileScopeManager {
    pub fn new() -> Self {
        Self {
            allowed_folders: Mutex::new(HashSet::new()),
        }
    }

    pub fn add_allowed_folder<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let canonical = path.as_ref().canonicalize().map_err(|e| e.to_string())?;
        if let Ok(mut folders) = self.allowed_folders.lock() {
            folders.insert(canonical);
            Ok(())
        } else {
            Err("Failed to lock folders".to_string())
        }
    }

    pub fn remove_allowed_folder<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let canonical = path.as_ref().canonicalize().map_err(|e| e.to_string())?;
        if let Ok(mut folders) = self.allowed_folders.lock() {
            folders.remove(&canonical);
            Ok(())
        } else {
            Err("Failed to lock folders".to_string())
        }
    }

    pub fn is_path_allowed<P: AsRef<Path>>(&self, target_path: P) -> bool {
        let canonical_target = match target_path.as_ref().canonicalize() {
            Ok(p) => p,
            Err(_) => return false,
        };

        if let Ok(folders) = self.allowed_folders.lock() {
            for folder in folders.iter() {
                if canonical_target.starts_with(folder) {
                    return true;
                }
            }
        }
        false
    }
}
