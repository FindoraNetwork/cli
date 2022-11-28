use {
    crate::server::Server,
    anyhow::{anyhow, Result},
    primitive_types::U256,
    serde_json::Value,
    std::str::FromStr,
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
    Ok(U256::from_str(val)?)
}
