mod asset;
mod commands;
mod server;
mod wallet;

use clap::Parser;

fn main() {
    let cli = commands::Cli::parse();

    cli.exeute().unwrap();
}
