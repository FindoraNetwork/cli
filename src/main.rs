use clap::Parser;

mod commands;

mod error;
pub use error::*;

fn main() {
    let cli = commands::Cli::parse();

    cli.exeute().unwrap();
}
