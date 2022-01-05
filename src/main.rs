use clap::{Parser, Subcommand};
use gnotes::config::load_config;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    New,
}

// TODO: use https://docs.rs/termimad/latest/termimad/ for show.
fn main() {
    let cli = Cli::parse();

    println!("cli {:?}", cli);

    // TODO: convert unwrap to `?`
    let home_dir = dirs::home_dir().expect("Unexpected error: home directory can't be located.");

    let config = load_config(&home_dir);
    println!("config {:?}", config);
}
