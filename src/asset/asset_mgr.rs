use {
    crate::{
        asset::{
            get_bridge_address, get_erc20_decimals, get_erc20_symbol, get_erc721_symbol,
            prism::{
                compute_erc20_asset_type, compute_nft_asset_type, get_asset_address,
                get_erc1155_tocken, get_erc20_tocken, get_erc721_tocken, get_prism_proxy_address,
                get_tocken_type, TokenType,
            },
            Asset, AssetType, ASSET_DIRECTORY,
        },
        chain_net::ChainNet,
    },
    anyhow::{anyhow, Result},
    ethabi::ethereum_types::{H160, U256},
    std::{
        collections::HashMap,
        fs::{create_dir_all, read_dir},
        path::Path,
    },
};

pub struct AssetMgr {
    pub home: String,
    pub assets: HashMap<String, Asset>,
}

impl AssetMgr {
    pub fn new(home_path: &str) -> Self {
        Self {
            home: String::from(home_path),
            assets: HashMap::new(),
        }
    }
    pub fn load_from_file(home: &str) -> Result<Self> {
        let asset_path = format!("{}/{}", home, ASSET_DIRECTORY);
        let asset_path = Path::new(asset_path.as_str());
        if !asset_path.exists() {
            create_dir_all(asset_path)?;
        }
        let mut assets = HashMap::new();
        for path in read_dir(asset_path)? {
            let file = path?.path();
            if !file.is_dir() {
                let asset = Asset::load_from_file(file.display().to_string().as_str())?;
                let key = asset.utxo_asset_code.clone();
                assets.insert(key, asset);
            }
        }
        Ok(Self {
            home: String::from(home),
            assets,
        })
    }

    pub fn add_utxo_asset(
        &mut self,
        chain_net: &ChainNet,
        utxo_asset_code: &str,
        utxo_decimals: u64,
        utxo_symbol: &str,
    ) -> Result<()> {
        let asset_path = format!("{}/{}", self.home.as_str(), ASSET_DIRECTORY);
        let asset_path = Path::new(asset_path.as_str());
        if !asset_path.exists() {
            create_dir_all(asset_path)?;
        }
        let url = format!(
            "{}:{}",
            chain_net.chain_net_address, chain_net.web3_rpc_port
        );
        let prism_proxy_address = get_prism_proxy_address(chain_net)?;
        let bridge_address = get_bridge_address(url.as_str(), prism_proxy_address)?;
        let asset_address = get_asset_address(url.as_str(), bridge_address)?;
        let tocken_type = get_tocken_type(url.as_str(), asset_address, utxo_asset_code)?;
        let mut asset = Asset::add(
            self.home.as_str(),
            utxo_asset_code,
            utxo_decimals,
            utxo_symbol,
            None,
            None,
            None,
            None,
            None,
        )?;
        match tocken_type {
            Some(TokenType::ERC20) => {
                if let Some(erc20_addr) =
                    get_erc20_tocken(url.as_str(), asset_address, utxo_asset_code)?
                {
                    let decimals = get_erc20_decimals(url.as_str(), erc20_addr)?;
                    let symbol = get_erc20_symbol(url.as_str(), erc20_addr)?;
                    asset.update_asset(
                        self.home.as_str(),
                        AssetType::FRC20,
                        erc20_addr,
                        None,
                        Some(decimals),
                        Some(symbol),
                    )?;
                }
            }
            Some(TokenType::ERC721) => {
                let (nft_address, tocken_id) =
                    get_erc721_tocken(url.as_str(), asset_address, utxo_asset_code)?;
                let symbol = get_erc721_symbol(url.as_str(), nft_address)?;
                asset.update_asset(
                    self.home.as_str(),
                    AssetType::FRC721,
                    nft_address,
                    Some(tocken_id),
                    None,
                    Some(symbol),
                )?;
            }
            Some(TokenType::ERC1155) => {
                let (nft_address, tocken_id) =
                    get_erc1155_tocken(url.as_str(), asset_address, utxo_asset_code)?;
                asset.update_asset(
                    self.home.as_str(),
                    AssetType::FRC1155,
                    nft_address,
                    Some(tocken_id),
                    None,
                    Some(asset.utxo_symbol.clone()),
                )?;
            }
            None => return Ok(()),
        };
        self.assets.insert(utxo_asset_code.to_string(), asset);
        Ok(())
    }

    pub fn add_evm_asset(
        &mut self,
        chain_net: &ChainNet,
        asset_type: AssetType,
        contract_address: H160,
        token_id: Option<U256>,
        utxo_symbol: &str,
    ) -> Result<()> {
        let asset_path = format!("{}/{}", self.home.as_str(), ASSET_DIRECTORY);
        let asset_path = Path::new(asset_path.as_str());
        if !asset_path.exists() {
            create_dir_all(asset_path)?;
        }

        let url = format!(
            "{}:{}",
            chain_net.chain_net_address, chain_net.web3_rpc_port
        );
        let prism_proxy_address = get_prism_proxy_address(chain_net)?;
        let bridge_address = get_bridge_address(url.as_str(), prism_proxy_address)?;

        match asset_type {
            AssetType::FRC20 => {
                let utxo_asset_code =
                    compute_erc20_asset_type(url.as_str(), bridge_address, contract_address)?;
                let utxo_asset_code = format!("0x{}", hex::encode(utxo_asset_code));
                let decimals = get_erc20_decimals(url.as_str(), contract_address)?;
                let symbol = get_erc20_symbol(url.as_str(), contract_address)?;
                let asset = Asset::add(
                    self.home.as_str(),
                    utxo_asset_code.as_str(),
                    decimals.as_u64(),
                    &symbol,
                    Some(AssetType::FRC20),
                    Some(contract_address),
                    None,
                    Some(decimals),
                    Some(symbol.to_string()),
                )?;
                self.assets.insert(utxo_asset_code.to_string(), asset);
                Ok(())
            }
            AssetType::FRC721 => {
                let token_id = match token_id {
                    Some(val) => val,
                    None => {
                        return Err(anyhow!("token_id not found"));
                    }
                };
                let utxo_asset_code = compute_nft_asset_type(
                    url.as_str(),
                    bridge_address,
                    contract_address,
                    token_id,
                )?;
                let utxo_asset_code = format!("0x{}", hex::encode(utxo_asset_code));
                let symbol = get_erc721_symbol(url.as_str(), contract_address)?;
                let asset = Asset::add(
                    self.home.as_str(),
                    utxo_asset_code.as_str(),
                    0,
                    &symbol,
                    Some(AssetType::FRC721),
                    Some(contract_address),
                    None,
                    None,
                    Some(symbol.to_string()),
                )?;
                self.assets.insert(utxo_asset_code.to_string(), asset);
                Ok(())
            }
            AssetType::FRC1155 => {
                let token_id = match token_id {
                    Some(val) => val,
                    None => {
                        return Err(anyhow!("token_id not found"));
                    }
                };
                let utxo_asset_code = compute_nft_asset_type(
                    url.as_str(),
                    bridge_address,
                    contract_address,
                    token_id,
                )?;
                let utxo_asset_code = format!("0x{}", hex::encode(utxo_asset_code));
                let asset = Asset::add(
                    self.home.as_str(),
                    utxo_asset_code.as_str(),
                    0,
                    utxo_symbol,
                    Some(AssetType::FRC1155),
                    Some(contract_address),
                    None,
                    None,
                    Some(utxo_symbol.to_string()),
                )?;
                self.assets.insert(utxo_asset_code.to_string(), asset);
                Ok(())
            }
        }
    }
}
