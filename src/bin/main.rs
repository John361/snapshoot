use lib::business::Initializer;
use lib::cli::Cli;
use lib::LOG_FILE;

fn main() {
    log4rs::init_file(LOG_FILE, Default::default())
        .unwrap_or_else(|_| panic!("Cannot init log4rs"));

    let cli = Cli::load();

    match cli {
        Cli::Shoot(args) => {
            let source = std::path::Path::new(&args.source);
            let destination = std::path::Path::new(&args.destination);

            Initializer::run(source, destination)
                .unwrap_or_else(|e| panic!("Error during initialization: {0}", e));

            println!("{:#?}", args);
        }
    }
}
