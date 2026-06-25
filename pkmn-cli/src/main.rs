use clap::Parser;
use pkmn_cli::Cli;

fn main() {
    let _ = dotenvy::dotenv();
    if let Err(error) = Cli::parse().run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
