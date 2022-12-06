mod asset;
mod chain_net;
mod commands;
mod wallet;

use clap::Parser;

fn main() {
    let cli = commands::Cli::parse();

    cli.exeute().unwrap();
}
