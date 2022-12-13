mod asset;
mod chain_net;
mod commands;
mod transfer;
mod txn_builder;
mod utils;
mod wallet;

use clap::Parser;

fn main() {
    let cli = commands::Cli::parse();

    cli.exeute().unwrap();
}
