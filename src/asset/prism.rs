use {
    crate::server::Server,
    anyhow::{anyhow, Result},
    ethabi::{Contract, Token},
    serde_json::Value,
    std::str::FromStr,
    tokio::runtime::Runtime,
    web3::{
        transports::Http,
        types::{BlockId, BlockNumber, Bytes, CallRequest, H160, U256},
        Web3,
    },
};

pub fn get_prism_proxy_address(server: &Server) -> Result<H160> {
    let url = format!(
        "{}:{}/display_checkpoint",
        server.server_address, server.query_port
    );
    let val = attohttpc::get(&url)
        .send()?
        .error_for_status()?
        .json::<Value>()?;
    let address = match val["prism_bridge_address"].as_str() {
        Some(val) => val,
        None => {
            return Err(anyhow!("prism_bridge_address json value not found"));
        }
    };
    Ok(H160::from_str(address)?)
}
pub fn get_bridge_address(url: &str, prism_proxy_address: H160) -> Result<H160> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismProxy.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi.function("prismBridgeAddress")?.encode_input(&[])?;
    let addr = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_proxy_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(H160::from_slice(&addr.0.to_vec()[12..]))
}

pub fn get_asset_address(url: &str, bridge_address: H160) -> Result<H160> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi.function("asset_contract")?.encode_input(&[])?;

    let addr = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(bridge_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(H160::from_slice(&addr.0.to_vec()[12..]))
}

pub fn compute_erc20_asset_type(
    url: &str,
    prism_address: H160,
    erc20_addr: H160,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("computeERC20AssetType")?
        .encode_input(&[Token::Address(erc20_addr)])?;

    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(bytes.0)
}

pub fn compute_nft_asset_type(
    url: &str,
    prism_address: H160,
    nft_address: H160,
    token_id: U256,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("computeNFTAssetType")?
        .encode_input(&[Token::Address(nft_address), Token::Uint(token_id)])?;
    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(bytes.0)
}
pub enum TokenType {
    ERC20,
    ERC721,
    ERC1155,
}
pub fn get_tocken_type(
    url: &str,
    asset_address: H160,
    asset_code: &str,
) -> Result<Option<TokenType>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXAsset.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let asset_code = if asset_code.starts_with("0x") {
        hex::decode(&asset_code[2..])?
    } else {
        hex::decode(asset_code)?
    };
    let data = abi
        .function("getTokenType")?
        .encode_input(&[Token::FixedBytes(asset_code)])?;
    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(asset_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    let asset_type = U256::from_str_radix(hex::encode(&bytes.0).as_str(), 16)?;
    let token_type = match asset_type.as_usize() {
        0 => Some(TokenType::ERC20),
        1 => Some(TokenType::ERC721),
        2 => Some(TokenType::ERC1155),
        _ => None,
    };
    Ok(token_type)
}

pub fn get_erc20_tocken(url: &str, asset_address: H160, asset_code: &str) -> Result<Option<H160>> {
    let data = get_tocken_info(url, asset_address, &asset_code, "getERC20Info")?;
    let addr = H160::from_slice(&data[12..]);

    if H160::default() == addr {
        Ok(None)
    } else {
        Ok(Some(H160::from_slice(&data[12..])))
    }
}

pub fn get_erc721_tocken(url: &str, asset_address: H160, asset_code: &str) -> Result<(H160, U256)> {
    let data = get_tocken_info(url, asset_address, asset_code, "getERC721Info")?;
    let tocken_id = U256::from_str_radix(hex::encode(&data[33..]).as_str(), 16)?;
    Ok((H160::from_slice(&data[12..32]), tocken_id))
}
pub fn get_erc1155_tocken(
    url: &str,
    asset_address: H160,
    asset_code: &str,
) -> Result<(H160, U256)> {
    let data = get_tocken_info(url, asset_address, asset_code, "getERC1155Info")?;
    let tocken_id = U256::from_str_radix(hex::encode(&data[33..]).as_str(), 16)?;
    Ok((H160::from_slice(&data[12..32]), tocken_id))
}

fn get_tocken_info(
    url: &str,
    asset_address: H160,
    asset_code: &str,
    func: &str,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXAsset.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let asset_code = if asset_code.starts_with("0x") {
        hex::decode(&asset_code[2..])?
    } else {
        hex::decode(asset_code)?
    };

    let data = abi
        .function(func)?
        .encode_input(&[Token::FixedBytes(asset_code)])?;

    let info = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(asset_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(info.0)
}