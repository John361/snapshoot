use lib::business::{Initializer, SnapshotBaseFolder, SnapshotProcess};
use lib::cli::Cli;
use lib::LOG_FILE;

#[tokio::main]
async fn main() {
    log4rs::init_file(LOG_FILE, Default::default())
        .unwrap_or_else(|_| panic!("Cannot init log4rs"));

    let cli = Cli::load();

    match cli {
        Cli::Shoot(args) => {
            let source = std::path::Path::new(&args.source);
            let destination = std::path::Path::new(&args.destination);

            let initializer = Initializer::default();
            initializer.run(source, destination)
                .unwrap_or_else(|e| panic!("Error during initialization: {0}", e));

            let snapshot_base_folder = SnapshotBaseFolder::default();
            let yesterday = snapshot_base_folder.get_yesterday(destination)
                .unwrap_or_else(|e| panic!("Cannot get yesterday snapshot folder even if does not exist: {0}", e));
            let today = snapshot_base_folder.create_today(destination)
                .unwrap_or_else(|e| panic!("Cannot create snapshot folder: {0}", e));
            let today = std::path::Path::new(&today);

            let snapshot_process = SnapshotProcess::default();
            snapshot_process.run(source, yesterday, today)
                .await
                .unwrap_or_else(|e| panic!("Cannot process to snapshot: {0}", e));
        }
    }
}
