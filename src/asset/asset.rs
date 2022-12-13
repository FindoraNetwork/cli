use {
    anyhow::Result,
    ethabi::ethereum_types::{H160, U256},
    serde::{Deserialize, Serialize},
    std::{
        fs::{read_to_string, File},
        io::Write,
    },
};

pub(crate) const ASSET_DIRECTORY: &str = "assets";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AssetType {
    FRC20,
    FRC721,
    FRC1155,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub utxo_asset_code: String,
    pub utxo_decimals: u64,
    pub utxo_symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<H160>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tocken_id: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl Asset {
    pub fn add(
        home_path: &str,
        utxo_asset_code: &str,
        utxo_decimals: u64,
        utxo_symbol: &str,
        asset_type: Option<AssetType>,
        contract_address: Option<H160>,
        tocken_id: Option<U256>,
        decimals: Option<U256>,
        symbol: Option<String>,
    ) -> Result<Self> {
        let asset = Asset {
            utxo_asset_code: String::from(utxo_asset_code),
            utxo_decimals,
            utxo_symbol: String::from(utxo_symbol),
            asset_type,
            contract_address,
            tocken_id,
            decimals,
            symbol,
        };
        let file_name = format!("{}/{}/{}.json", home_path, ASSET_DIRECTORY, utxo_asset_code);
        let mut file = File::create(file_name)?;
        file.write_all(serde_json::to_string(&asset)?.as_bytes())?;
        Ok(asset)
    }
    pub fn update_asset(
        &mut self,
        home_path: &str,
        asset_type: AssetType,
        contract_address: H160,
        tocken_id: Option<U256>,
        decimals: Option<U256>,
        symbol: Option<String>,
    ) -> Result<()> {
        self.asset_type = Some(asset_type);
        self.contract_address = Some(contract_address);
        self.tocken_id = tocken_id;
        self.decimals = decimals;
        self.symbol = symbol;
        let file_name = format!(
            "{}/{}/{}.json",
            home_path, ASSET_DIRECTORY, self.utxo_asset_code
        );
        let mut file = File::create(file_name)?;
        file.write_all(serde_json::to_string(&self)?.as_bytes())?;
        Ok(())
    }
    pub fn load_from_file(file_name: &str) -> Result<Self> {
        let json = read_to_string(file_name)?;
        let asset = serde_json::from_str::<Self>(json.as_str())?;
        Ok(asset)
    }
}
