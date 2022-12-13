use {
    crate::{
        asset::{show_eth_address, show_evm_address, show_fra_address, AssetMgr, AssetType},
        chain_net::ChainNet,
        wallet::{AccountMgr, AccountType},
    },
    anyhow::Result,
    clap::{CommandFactory, Parser},
    ethabi::ethereum_types::{H160, U256},
};

#[derive(Parser)]
///Asset Management
pub struct Asset {
    ///show address asset
    #[arg(
        short,
        long,
        conflicts_with = "add",
        conflicts_with = "typ",
        conflicts_with = "asset",
        conflicts_with = "decimals",
        conflicts_with = "symbol",
        conflicts_with = "contract_address",
        conflicts_with = "token_id"
    )]
    show: bool,
    ///the address of the asset to display
    #[arg(
        long,
        conflicts_with = "add",
        conflicts_with = "typ",
        conflicts_with = "asset",
        conflicts_with = "decimals",
        conflicts_with = "symbol",
        conflicts_with = "contract_address",
        conflicts_with = "token_id"
    )]
    address: Option<String>,
    ///add asset type
    #[arg(short, long, conflicts_with = "address", conflicts_with = "show")]
    add: bool,
    ///asset type(utxo,frc20,frc721,frc1155)
    #[arg(
        short,
        long = "type",
        value_name = "TYPE",
        conflicts_with = "address",
        conflicts_with = "show"
    )]
    typ: Option<String>,
    ///asset hex code
    #[arg(long, conflicts_with = "address", conflicts_with = "show")]
    asset: Option<String>,
    #[arg(long, conflicts_with = "address", conflicts_with = "show")]
    decimals: Option<u64>,
    #[arg(long, conflicts_with = "address", conflicts_with = "show")]
    symbol: Option<String>,
    #[arg(long, conflicts_with = "address", conflicts_with = "show")]
    contract_address: Option<H160>,
    #[arg(long, conflicts_with = "address", conflicts_with = "show")]
    token_id: Option<U256>,
}

impl Asset {
    pub fn execute(self, chain_net: &ChainNet, home: &str) -> Result<()> {
        if self.show {
            let account_mgr = match AccountMgr::load_from_file(home) {
                Ok(val) => val,
                Err(e) => {
                    println!("load account error: {:?}", e);
                    return Ok(());
                }
            };
            let asset_mgr = match AssetMgr::load_from_file(home) {
                Ok(val) => val,
                Err(e) => {
                    println!("load account error: {:?}", e);
                    return Ok(());
                }
            };
            let address = match self.address.as_deref() {
                Some(val) => val,
                None => {
                    println!("address is required");
                    return Ok(());
                }
            };
            let account = match account_mgr.accounts.get(address) {
                Some(acc) => acc.clone(),
                None => {
                    println!("account {} not found", address);
                    return Ok(());
                }
            };
            println!("\n\x1b[31;01mAddress:\x1b[00m {}", address);
            if !address.starts_with("0x") {
                match account.account_type {
                    AccountType::Fra => {
                        let kp = match account.get_key_pair() {
                            Ok(val) => val,
                            Err(e) => {
                                println!("account {} key pair error:{}", address, e);
                                return Ok(());
                            }
                        };
                        if let Err(e) = show_fra_address(&chain_net, address, &kp, &asset_mgr) {
                            println!("show address balance error:{}", e);
                        };
                    }
                    AccountType::Eth => {
                        let kp = match account.get_key_pair() {
                            Ok(val) => val,
                            Err(e) => {
                                println!("account {} key pair error:{}", address, e);
                                return Ok(());
                            }
                        };
                        if let Err(e) = show_eth_address(&chain_net, address, &kp, &asset_mgr) {
                            println!("show address balance error:{}", e);
                        };
                    }
                    AccountType::Evm => {
                        if let Err(e) = show_evm_address(&chain_net, address, &asset_mgr) {
                            println!("show address balance error:{}", e);
                        };
                    }
                }
            } else {
                if let Err(e) = show_evm_address(&chain_net, address, &asset_mgr) {
                    println!("show address balance error:{}", e);
                };
            }
            return Ok(());
        } else if self.add {
            let asset_type = match self.typ.as_deref() {
                Some("utxo") => None,
                Some("frc20") => Some(AssetType::FRC20),
                Some("frc721") => Some(AssetType::FRC721),
                Some("frc1155") => Some(AssetType::FRC1155),
                _ => {
                    println!("type not support!!!");
                    return Ok(());
                }
            };
            let utxo_asset_code = match self.asset.as_deref() {
                Some(val) => {
                    if val.starts_with("0x") {
                        val
                    } else {
                        println!("asset_codemust be start with 0x !!!");
                        return Ok(());
                    }
                }
                None => {
                    if asset_type.is_none() {
                        println!("asset_code not found!!!");
                        return Ok(());
                    } else {
                        ""
                    }
                }
            };
            let utxo_symbol = self.symbol.as_deref().unwrap_or("");
            if Some(AssetType::FRC1155) == asset_type || asset_type.is_none() {
                if utxo_symbol.is_empty() {
                    println!("symbol not found!!!");
                    return Ok(());
                }
            }
            let utxo_decimals = self.decimals.unwrap_or(0);

            let contract_address = if asset_type.is_some() {
                match self.contract_address {
                    Some(val) => val,
                    None => {
                        println!("contract_address not found!!!");
                        return Ok(());
                    }
                }
            } else {
                H160::default()
            };
            let token_id = if Some(AssetType::FRC721) == asset_type
                || Some(AssetType::FRC1155) == asset_type
            {
                match self.token_id {
                    Some(val) => Some(val),
                    None => {
                        println!("token_id not found!!!");
                        return Ok(());
                    }
                }
            } else {
                None
            };
            let mut mgr = AssetMgr::new(home);

            if let Err(e) = match asset_type {
                Some(val) => {
                    mgr.add_evm_asset(&chain_net, val, contract_address, token_id, utxo_symbol)
                }
                None => mgr.add_utxo_asset(&chain_net, utxo_asset_code, utxo_decimals, utxo_symbol),
            } {
                println!("add asset error:{}", e);
            } else {
                println!("add asset success");
            }
        } else {
            Self::command().print_help()?;
        }
        Ok(())
    }
}
