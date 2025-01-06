use std::io::SeekFrom;
use std::path::{Path, PathBuf};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

pub struct ShootStructureGenerator {
    pub source: String,
    pub destination: String,
    source_size: u64,
}

impl Default for ShootStructureGenerator {
    fn default() -> Self {
        let source = "/tmp/.snapshoot-tests/source".to_string();
        let destination = "/tmp/.snapshoot-tests/destination".to_string();

        ShootStructureGenerator {
            source,
            destination,
            source_size: 128 * 1024 * 1024,
        }
    }
}

impl ShootStructureGenerator {
    pub async fn generate_source_folder(&self) {
        let path = Path::new(&self.source);
        fs::create_dir_all(path).await.unwrap();

        let mut total_size = 0;
        while total_size < self.source_size {
            let added_size = self.random_structure(path, &mut total_size).await;
            total_size += added_size;
        }
    }

    pub async fn generate_destination_folder(&self) {
        let path = Path::new(&self.destination);
        fs::create_dir_all(path).await.unwrap();
    }

    pub async fn clean_generated_folders(&self) {
        fs::remove_dir_all(&self.source).await.unwrap();
        fs::remove_dir_all(&self.destination).await.unwrap();
    }

    async fn random_structure(&self, base_path: &Path, total_size: &mut u64) -> u64 {
        let mut rng = rand::thread_rng();
        let mut size_added = 0;
        let num_items = rng.gen_range(5..15);

        for _ in 0..num_items {
            if rng.gen_bool(0.5) {
                let dir_path = base_path.join(format!("folder_{}", rng.gen::<u32>()));
                fs::create_dir_all(&dir_path).await.unwrap();
            } else {
                let file_path = base_path.join(format!("file_{}", rng.gen::<u32>()));

                if rng.gen_bool(0.2) {
                    let target_file = self.find_random_file(base_path).await.unwrap_or(file_path.clone());
                    fs::symlink(&target_file, &file_path).await.unwrap();
                } else {
                    let file_size = rng.gen_range(1 * 1024 * 1024..10 * 1024 * 1024); // 1 MB à 10 MB
                    size_added += self.create_random_file(&file_path, file_size).await;
                    *total_size += file_size as u64;
                }
            }
        }

        if rng.gen_bool(0.3) {
            let sub_dir = base_path.join(format!("subfolder_{}", rng.gen::<u32>()));
            fs::create_dir_all(&sub_dir).await.unwrap();

            let result = Box::pin(self.random_structure(&sub_dir, total_size)).await;
            size_added += result;
        }

        size_added
    }

    async fn create_random_file(&self, file_path: &Path, size: usize) -> u64 {
        let mut file = File::create(file_path).await.unwrap();
        file.set_len(size as u64).await.unwrap();
        file.seek(SeekFrom::Start(0)).await.unwrap();

        let mut rng = rand::thread_rng();
        let buffer: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
        file.write_all(&buffer).await.unwrap();

        size as u64
    }

    async fn find_random_file(&self, base_path: &Path) -> Option<PathBuf> {
        let mut rng = StdRng::from_entropy();
        let mut entries = match fs::read_dir(base_path).await {
            Ok(entries) => entries,
            Err(_) => return None,
        };

        let mut file_paths = Vec::new();
        while let Some(entry) = entries.next_entry().await.unwrap() {
            if entry.path().is_file() {
                file_paths.push(entry.path());
            }
        }

        if file_paths.is_empty() {
            None
        } else {
            Some(file_paths[rng.gen_range(0..file_paths.len())].clone())
        }
    }
}
