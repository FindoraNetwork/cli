use {
    anyhow::{anyhow, Result},
    ethabi::{ethereum_types::U256, Contract, ParamType, Token},
    tokio::runtime::Runtime,
    web3::{
        transports::Http,
        types::{BlockId, BlockNumber, Bytes, CallRequest, H160},
        Web3,
    },
};

pub fn get_erc20_symbol(url: &str, contract_address: H160) -> Result<String> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);

    let json = include_str!("./abi/ERC20.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi.function("symbol")?.encode_input(&[])?;
    let balance = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::String], &balance.0)?;
    if let Some(Token::String(symbol)) = ret.get(0) {
        Ok(symbol.clone())
    } else {
        Err(anyhow!("symbol not found"))
    }
}

pub fn get_erc20_decimals(url: &str, contract_address: H160) -> Result<U256> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);

    let json = include_str!("./abi/ERC20.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi.function("decimals")?.encode_input(&[])?;
    let decimals = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::Uint(256)], &decimals.0)?;
    let decimals = if let Some(Token::Uint(decimals)) = ret.get(0) {
        decimals.clone()
    } else {
        return Err(anyhow!("decimals not found"));
    };
    Ok(decimals)
}

pub fn call_erc20_balance_of(url: &str, address: H160, contract_address: H160) -> Result<U256> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/ERC20.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("balanceOf")?
        .encode_input(&[Token::Address(address)])?;
    let balance = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::Uint(256)], &balance.0)?;
    let balance = if let Some(Token::Uint(balance)) = ret.get(1) {
        balance.clone()
    } else {
        return Err(anyhow!("balance not found"));
    };
    Ok(balance)
}

pub fn get_erc721_symbol(url: &str, contract_address: H160) -> Result<String> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);

    let json = include_str!("./abi/ERC721.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi.function("symbol")?.encode_input(&[])?;
    let balance = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::String], &balance.0)?;
    if let Some(Token::String(symbol)) = ret.get(0) {
        Ok(symbol.clone())
    } else {
        Err(anyhow!("symbol not found"))
    }
}

pub fn call_erc721_balance_of(
    url: &str,
    address: H160,
    contract_address: H160,
) -> Result<Option<U256>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/ERC721.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("balanceOf")?
        .encode_input(&[Token::Address(address)])?;
    let balance = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::Uint(256)], &balance.0)?;
    if let Some(Token::Uint(balance)) = ret.get(0) {
        Ok(Some(balance.clone()))
    } else {
        Ok(None)
    }
}

pub fn call_erc1155_balance_of(
    url: &str,
    address: H160,
    token_id: U256,
    contract_address: H160,
) -> Result<U256> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/ERC1155.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("balanceOf")?
        .encode_input(&[Token::Address(address), Token::Uint(token_id)])?;
    let balance = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(contract_address),
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
    let ret = ethabi::decode(&[ParamType::Uint(256)], &balance.0)?;
    if let Some(Token::Uint(balance)) = ret.get(0) {
        Ok(balance.clone())
    } else {
        Err(anyhow!("balance not found"))
    }
}
