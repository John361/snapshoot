use std::ffi::OsStr;
use std::fs::{read_dir, set_permissions, OpenOptions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

static EXTENSION: &str = "snapshoot";

pub struct Initializer;

impl Initializer {
    pub fn run(source: &Path, destination: &Path) -> Result<(), String> {
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
        let snapshoot_file = format!(".{0}.{1}", snapshoot_file, EXTENSION);
        let snapshoot_file_path = destination.join(&snapshoot_file);

        if Self::another_initialization(destination, &snapshoot_file)? {
            return Err("Destination folder already initialized for another folder".to_string());
        }

        if !snapshoot_file_path.exists() {
            Self::create_snapshoot(&snapshoot_file_path)?;
            log::info!("Snapshoot successfully initialized");
        } else {
            log::info!("Snapshoot already initialized")
        }

        Ok(())
    }

    fn create_snapshoot(path: &Path) -> Result<(), String> {
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        set_permissions(path, Permissions::from_mode(0o400))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn another_initialization(path: &Path, current_snapshoot_file: &str) -> Result<bool, String> {
        let folder = read_dir(path)
            .map_err(|e| e.to_string())?;

        for entry in folder {
            let entry = entry
                .map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() && path.extension() == Some(OsStr::new(EXTENSION)) {
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
