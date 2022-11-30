use {
    crate::server::Server,
    anyhow::{anyhow, Result},
    ethabi::Token,
    primitive_types::U256,
    serde_json::Value,
    std::str::FromStr,
    web3::{
        transports::Http,
        types::{BlockId, BlockNumber, Bytes, CallRequest, H160},
        Web3,
    },
};

pub fn get_evm_balance(server: &Server, addr: &str) -> Result<U256> {
    let url = format!("{}:{}", server.server_address, server.web3_rpc_port);
    let json = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":\"anything\",\"method\":\"eth_getBalance\",\"params\":[\"{}\",\"latest\"]}}",
        addr
    );
    let byte = attohttpc::post(url)
        .header("Content-Type", "application/json")
        .text(&json)
        .send()?
        .error_for_status()?
        .bytes()?;
    let val = serde_json::from_slice::<Value>(&byte)?;
    let val = val["result"]
        .as_str()
        .ok_or_else(|| anyhow!("result not fount"))?;
    Ok(U256::from_str_radix(val, 16)?)
}

pub fn call_balance_of(server: &Server, address: &str, contract_address: &str) -> Result<U256> {
    let json = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]".as_bytes();
    let url = format!(
        "{}:{}",
        server.server_address.as_str(),
        server.web3_rpc_port
    );
    let transport = Http::new(url.as_str()).unwrap();
    let web3 = Web3::new(transport);
    let contract_address = H160::from_str(contract_address)?;
    let address = H160::from_str(address)?;
    let abi = ethabi::Contract::load(json).unwrap();
    let data = abi
        .function("balanceOf")?
        .encode_input(&[Token::Address(address)])?;
    let balance = futures::executor::block_on(web3.eth().call(
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
    let val = String::from_utf8_lossy(&balance.0).to_string();
    Ok(U256::from_str_radix(val.as_str(), 16)?)
}
