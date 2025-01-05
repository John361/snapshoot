use std::fs::create_dir;
use std::path::{Path, PathBuf};

use chrono::Local;

pub struct SnapshotBaseFolder {
    date_format: String,
}

impl Default for SnapshotBaseFolder {
    fn default() -> Self {
        SnapshotBaseFolder {
            date_format: "%Y-%m-%d".to_string(),
        }
    }
}

impl SnapshotBaseFolder {
    pub fn create_today(&self, base_path: &Path) -> Result<PathBuf, String> {
        let path = self.create_dir(base_path)?;
        Ok(path)
    }

    pub fn get_yesterday(&self, base_path: &Path) -> Result<Option<PathBuf>, String> {
        let yesterday = Local::now()
            .date_naive()
            .pred_opt()
            .ok_or_else(|| "Cannot get yesterday date".to_string())?
            .format(&self.date_format)
            .to_string();

        let folder = base_path.join(yesterday);

        if folder.exists() {
           Ok(Some(folder))
        } else {
            Ok(None)
        }
    }

    fn create_dir(&self, path: &Path) -> Result<PathBuf, String> {
        let today = Local::now()
            .format(&self.date_format)
            .to_string();

        let folder = path.join(today);
        create_dir(&folder)
            .map_err(|e| format!("Failed to create snapshot folder: {}", e))?;

        log::info!("Snapshot folder '{}' created", folder.display());
        Ok(folder)
    }
}
