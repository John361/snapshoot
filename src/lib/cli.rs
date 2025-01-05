mod shoot;

pub use shoot::*;

use clap::Parser;

use crate::APP_NAME;

#[derive(Parser)]
#[command(
    version,
    name = APP_NAME,
    bin_name = APP_NAME
)]
pub enum Cli {
    #[command(about = "Shoot process")]
    Shoot(ShootArgs),
}

impl Cli {
    pub fn load() -> Self {
        Cli::parse()
    }
}
