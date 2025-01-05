use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct SnapshotProcess;

impl SnapshotProcess {
    pub fn run(&self, source: &Path, yesterday: Option<PathBuf>, today: &Path) -> Result<(), String> {
        if yesterday.is_none() {
            self.run_fresh(source, today)
        } else {
            let yesterday = yesterday.unwrap();
            let yesterday = Path::new(&yesterday);
            self.run_on_existing(source, yesterday, today)
        }
    }

    fn run_fresh(&self, _source: &Path, _today: &Path) -> Result<(), String> {
        log::info!("Running fresh snapshot");
        Ok(())
    }

    fn run_on_existing(&self, _source: &Path, _yesterday: &Path, _today: &Path) -> Result<(), String> {
        log::info!("Running with existing snapshot");
        Ok(())
    }
}
