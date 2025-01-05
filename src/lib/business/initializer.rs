use std::ffi::OsStr;
use std::fs::{read_dir, set_permissions, OpenOptions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct Initializer {
    file_extension: String,
}

impl Default for Initializer {
    fn default() -> Self {
        Self {
            file_extension: "snapshoot".to_string(),
        }
    }
}

impl Initializer {
    pub fn run(&self, source: &Path, destination: &Path) -> Result<(), String> {
        if !source.is_absolute() || !destination.is_absolute() {
            return Err("Source and destination folder must be an absolute path".to_string());
        }

        if !source.exists() || !destination.exists() {
            return Err("Source and destination folder must exists".to_string());
        }

        if !source.is_dir() || !destination.is_dir() {
            return Err("Source and destination folder must be a directory".to_string());
        }

        let snapshoot_file = source
            .file_name().unwrap()
            .to_str().unwrap();
        let snapshoot_file = format!(".{0}.{1}", snapshoot_file, &self.file_extension);
        let snapshoot_file_path = destination.join(&snapshoot_file);

        if self.another_initialization(destination, &snapshoot_file)? {
            return Err("Destination folder already initialized for another folder".to_string());
        }

        if !snapshoot_file_path.exists() {
            if !read_dir(destination)
                .map_err(|e| e.to_string())?
                .next()
                .is_none() {
                return Err("Destination folder must be empty".to_string());
            }

            self.create_snapshoot(&snapshoot_file_path)?;
            log::info!("Snapshoot successfully initialized");
        } else {
            log::info!("Snapshoot already initialized")
        }

        Ok(())
    }

    fn create_snapshoot(&self, path: &Path) -> Result<(), String> {
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        set_permissions(path, Permissions::from_mode(0o400))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn another_initialization(&self, path: &Path, current_snapshoot_file: &str) -> Result<bool, String> {
        let folder = read_dir(path)
            .map_err(|e| e.to_string())?;

        for entry in folder {
            let entry = entry
                .map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() && path.extension() == Some(OsStr::new(&self.file_extension)) {
                let filename = path
                    .file_name().unwrap()
                    .to_str().unwrap();

                if filename != current_snapshoot_file {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
