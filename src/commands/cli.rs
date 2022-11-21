use clap::{Parser, Subcommand};

use crate::Result;

use super::{Transfer, Asset};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Findora high level cli.
pub struct Cli {
    #[arg(short = 'H', long)]
    /// Set home dir
    home: String,

    #[arg(short, long)]
    /// Enable info log level
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn exeute(self) -> Result<()> {
        match self.command {
            Commands::Transfer(c) => c.execute()?,
            Commands::Asset(c) => c.execute()?,
        }

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Transfer(Transfer),
    Asset(Asset),
}
