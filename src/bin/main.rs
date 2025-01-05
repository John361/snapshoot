use lib::LOG_FILE;

fn main() {
    log4rs::init_file(LOG_FILE, Default::default())
        .unwrap_or_else(|_| panic!("Cannot init log4rs"));

    log::info!("Hello, world!");
}
