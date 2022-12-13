use {
    anyhow::{anyhow, Result},
    ethabi::ethereum_types::{H160, U256},
    serde_json::Value,
};

pub fn get_evm_balance(url: &str, addr: H160) -> Result<U256> {
    let json = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":\"anything\",\"method\":\"eth_getBalance\",\"params\":[\"{:?}\",\"latest\"]}}",
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
    let val = if val.starts_with("0x") {
        &val[2..]
    } else {
        val
    };
    Ok(U256::from_str_radix(val, 16)?)
}
