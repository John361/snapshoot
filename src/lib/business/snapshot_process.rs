use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tokio::fs;

#[derive(Default)]
pub struct SnapshotProcess;

impl SnapshotProcess {
    pub async fn run(&self, source: &Path, yesterday: Option<PathBuf>, today: &Path) -> Result<(), String> {
        if yesterday.is_none() {
            self.run_fresh(source, today).await
        } else {
            let yesterday = yesterday.unwrap();
            let yesterday = Path::new(&yesterday);
            self.run_on_existing(source, yesterday, today).await
        }
    }

    async fn run_fresh(&self, source: &Path, today: &Path) -> Result<(), String> {
        let mut entries = fs::read_dir(source)
            .await
            .map_err(|e| format!("Failed to read source directory: {0}", e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| e.to_string())? {

            let path = entry.path();
            let destination = today.join(entry.file_name());

            let path_file_name = path.file_name()
                .ok_or_else(|| format!("Failed to get folder name from path: {:?}", path))?;
            let path_file = today.join(path_file_name);

            if path.is_dir() {
                std::fs::create_dir(&path_file)
                    .map_err(|e| format!("Failed to create folder: {0}", e))?;

                Box::pin(self.run_fresh(&path, &destination)).await?;
            } else if path.is_symlink() {
                let link = fs::read_link(path)
                    .await
                    .map_err(|e| format!("Failed to read symlink target: {0}", e))?;

                fs::symlink(&link, path_file)
                    .await
                    .map_err(|e| format!("Failed to create symlink in destination: {0}", e))?;
            } else {
                fs::copy(&path, &destination)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    async fn run_on_existing(&self, source: &Path, yesterday: &Path, today: &Path) -> Result<(), String> {
        let mut entries = fs::read_dir(source)
            .await
            .map_err(|e| format!("Failed to read source directory: {0}", e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| e.to_string())? {

            let path = entry.path();
            let yesterday_destination = yesterday.join(entry.file_name());

            if yesterday_destination.exists() {
                if path.is_file() {
                    let path_file_name = path.file_name()
                        .ok_or_else(|| format!("Failed to get folder name from path: {:?}", path))?;
                    let today_path_file = today.join(path_file_name);

                    if self.same_hash(&yesterday_destination, &path).await? {
                        fs::symlink(&yesterday_destination, today_path_file)
                            .await
                            .map_err(|e| format!("Failed to create symlink in destination: {0}", e))?;
                    } else {
                        // Box::pin(self.run_fresh(&path, today)).await?;
                    }
                } else {
                    // Box::pin(self.run_fresh(&path, today)).await?;
                }
            }
        }

        Ok(())
    }

    async fn same_hash(&self, source: &Path, destination: &Path) -> Result<bool, String> {
        let source_hash = self.calculate_hash(source).await?;
        let destination_hash = self.calculate_hash(destination).await?;

        Ok(source_hash == destination_hash)
    }

    async fn calculate_hash(&self, path: &Path) -> Result<String, String> {
        let mut file = std::fs::File::open(path)
            .map_err(|e| format!("Failed to open file for hash calculation: {0}", e))?;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 4096]; // Buffer de 4 Ko

        loop {
            let n = file.read(&mut buffer).map_err(|e| format!("Failed to read file: {0}", e))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }
}
