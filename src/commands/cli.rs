use {
    crate::{
        chain_net::ChainNetMgr,
        commands::{Asset, ChainNet, Transfer, Wallet},
    },
    anyhow::Result,
    clap::{Parser, Subcommand},
    env_logger::Env,
    std::{fs::create_dir_all, path::Path},
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
    #[arg(
        long,
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    mainnet: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    testnet: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    forge: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    qa01: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    qa02: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa04",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    qa03: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "local",
        conflicts_with = "custom_network_name"
    )]
    qa04: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "custom_network_name"
    )]
    local: bool,
    #[arg(
        long,
        conflicts_with = "mainnet",
        conflicts_with = "testnet",
        conflicts_with = "forge",
        conflicts_with = "qa01",
        conflicts_with = "qa02",
        conflicts_with = "qa03",
        conflicts_with = "qa04",
        conflicts_with = "local"
    )]
    custom_network_name: Option<String>,

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
            println!("home path not a folder");
            return Ok(());
        }
        let chain_net_name = if self.mainnet {
            Some("mainnet")
        } else if self.testnet {
            Some("testnet")
        } else if self.forge {
            Some("forge")
        } else if self.qa01 {
            Some("qa01")
        } else if self.qa02 {
            Some("qa02")
        } else if self.qa03 {
            Some("qa03")
        } else if self.qa04 {
            Some("qa04")
        } else {
            match self.custom_network_name.as_deref() {
                Some(v) => Some(v),
                None => None,
            }
        };

        match self.command {
            Commands::ChainNet(c) => c.execute(home.as_str())?,
            Commands::Wallet(c) => {
                let mgr = match ChainNetMgr::load_all(home.as_str()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("load chain_net error:{}", e);
                        return Ok(());
                    }
                };
                let chain_net = match chain_net_name {
                    Some(val) => match mgr.chain_nets.get(val) {
                        Some(v) => v,
                        None => {
                            println!("chain_net not found");
                            return Ok(());
                        }
                    },
                    None => {
                        println!("chain_net not found");
                        return Ok(());
                    }
                };

                c.execute(chain_net, home.as_str())?
            }
            Commands::Asset(c) => {
                let mgr = match ChainNetMgr::load_all(home.as_str()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("load chain_net error:{}", e);
                        return Ok(());
                    }
                };
                let chain_net = match chain_net_name {
                    Some(val) => match mgr.chain_nets.get(val) {
                        Some(v) => v,
                        None => {
                            println!("chain_net not found");
                            return Ok(());
                        }
                    },
                    None => {
                        println!("chain_net not found");
                        return Ok(());
                    }
                };

                c.execute(chain_net, home.as_str())?
            }
            Commands::Transfer(c) => {
                let mgr = match ChainNetMgr::load_all(home.as_str()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("load chain_net error:{}", e);
                        return Ok(());
                    }
                };
                let chain_net = match chain_net_name {
                    Some(val) => match mgr.chain_nets.get(val) {
                        Some(v) => v,
                        None => {
                            println!("chain_net not found");
                            return Ok(());
                        }
                    },
                    None => {
                        println!("chain_net not found");
                        return Ok(());
                    }
                };
                c.execute(chain_net, home.as_str())?
            }
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    ChainNet(ChainNet),
    Wallet(Wallet),
    Asset(Asset),
    Transfer(Transfer),
}
