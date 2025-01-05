use std::path::{Path, PathBuf};

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
            .map_err(|e| format!("Failed to read source directory: {}", e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| e.to_string())? {

            let path = entry.path();
            let destination = today.join(entry.file_name());

            if path.is_dir() {
                let folder_name = path.file_name()
                    .ok_or_else(|| format!("Failed to get folder name from path: {:?}", path))?;
                let folder = today.join(folder_name);

                std::fs::create_dir(&folder)
                    .map_err(|e| format!("Failed to create folder: {}", e))?;

                self.run_fresh_recursively(&path, &destination).await?;
            } else {
                fs::copy(&path, &destination)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    async fn run_fresh_recursively(&self, source: &Path, today: &Path) -> Result<(), String> {
        Box::pin(self.run_fresh(source, today)).await
    }

    async fn run_on_existing(&self, _source: &Path, _yesterday: &Path, _today: &Path) -> Result<(), String> {
        Ok(())
    }
}
