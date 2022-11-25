use {
    super::{Asset, Transfer, Wallet},
    anyhow::{anyhow, Result},
    clap::{Parser, Subcommand},
    std::fs::create_dir_all,
    std::path::Path,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Findora high level cli.
pub struct Cli {
    #[arg(short = 'H', long)]
    /// Set home dir
    home: Option<String>,

    #[arg(short, long)]
    /// Enable info log level
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn exeute(self) -> Result<()> {
        let home = self
            .home
            .unwrap_or(format!("{}/.findora_cli/", std::env::var("HOME")?));

        let home_path = Path::new(home.as_str());
        if !home_path.exists() {
            create_dir_all(home_path)?;
        } else if !home_path.is_dir() {
            return Err(anyhow!("home path not a folder"));
        }

        match self.command {
            Commands::Wallet(c) => c.execute(home.as_str())?,
            Commands::Asset(c) => c.execute(home.as_str())?,
            Commands::Transfer(c) => c.execute(home.as_str())?,
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Wallet(Wallet),
    Asset(Asset),
    Transfer(Transfer),
}
