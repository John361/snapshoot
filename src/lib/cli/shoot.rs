use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Shoot arguments", long_about = None)]
pub struct ShootArgs {
    #[arg(long, help = "Source folder (required)", required = true)]
    pub source: String,

    #[arg(long, help = "Destination folder (required)", required = true)]
    pub destination: String,
}
