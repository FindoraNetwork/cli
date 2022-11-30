use {
    super::{Asset, Server, Transfer, Wallet},
    anyhow::{anyhow, Result},
    clap::{Parser, Subcommand},
    env_logger::Env,
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
        let env_log = if self.verbose {
            Env::new().default_filter_or("trace")
        } else {
            Env::new().default_filter_or("off")
        };
        env_logger::try_init_from_env(env_log)?;

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
            Commands::Server(c) => c.execute(home.as_str())?,
            Commands::Wallet(c) => c.execute(home.as_str())?,
            Commands::Asset(c) => c.execute(home.as_str())?,
            Commands::Transfer(c) => c.execute(home.as_str())?,
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Server(Server),
    Wallet(Wallet),
    Asset(Asset),
    Transfer(Transfer),
}
