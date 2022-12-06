use {
    crate::{
        asset::{
            call_erc1155_balance_of, call_erc20_balance_of, call_erc721_balance_of,
            get_evm_balance, get_owned_utxo_balance, AssetMgr, AssetType,
        },
        chain_net::ChainNet,
    },
    anyhow::{anyhow, Result},
    ethabi::ethereum_types::{H160, U256},
    noah::xfr::sig::{convert_libsecp256k1_public_key_to_address, XfrKeyPair, XfrPublicKeyInner},
    std::{collections::HashMap, str::FromStr},
};

const FRA_ASSET_CODE: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

pub fn show_evm_address(chain_net: &ChainNet, address: &str, mgr: &AssetMgr) -> Result<()> {
    let url = format!(
        "{}:{}",
        chain_net.chain_net_address, chain_net.web3_rpc_port
    );
    let address = H160::from_str(address)?;
    let bar_balance = U256::zero();
    let abar_balance = U256::zero();
    let mut evm_balance = match get_evm_balance(&url, address) {
        Ok(balance) => balance,
        Err(e) => {
            println!("account {} get_evm_balance error:{}", address, e);
            return Ok(());
        }
    };
    println!(
        "========================================================================================"
    );
    println!("{} FRA", bar_balance + abar_balance + evm_balance,);
    println!("- {} FRA(BAR)", bar_balance);
    println!("- {} FRA(ABAR)", abar_balance);
    println!("- {} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
        match asset.asset_type {
            Some(AssetType::FRC20) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance = call_erc20_balance_of(url.as_str(), address, contract_address)?;
                if U256::zero() == bar_balance + abar_balance + evm_balance {
                    continue;
                }
                println!("========================================================================================");
                println!(
                    "{} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(FRC20,{:?})",
                    evm_balance,
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            Some(AssetType::FRC721) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                let tocken_id =
                    match call_erc721_balance_of(url.as_str(), address, contract_address)? {
                        Some(val) => val,
                        None => continue,
                    };
                println!("========================================================================================");
                println!("{: <4} tocken id:{} ", symbol, tocken_id);
                println!(
                    "- {: >10} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- 1 {: <4}(FRC721,{:?})",
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            Some(AssetType::FRC1155) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                let tocken_id = match asset.tocken_id {
                    Some(val) => val,
                    None => continue,
                };
                evm_balance =
                    call_erc1155_balance_of(url.as_str(), address, tocken_id, contract_address)?;
                println!("========================================================================================");
                println!(
                    "{} {: <4} tocken id:{}",
                    bar_balance + abar_balance + evm_balance,
                    symbol,
                    tocken_id
                );
                println!(
                    "- {} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(FRC1155,{:?})",
                    evm_balance,
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            _ => {}
        };
    }
    Ok(())
}
pub fn show_fra_address(
    chain_net: &ChainNet,
    address: &str,
    kp: &XfrKeyPair,
    mgr: &AssetMgr,
) -> Result<()> {
    let owned_utxo = match get_owned_utxo_balance(&chain_net, &kp) {
        Ok(utxo) => utxo,
        Err(e) => {
            return Err(anyhow!("account {} get_owned_utxos error:{}", address, e));
        }
    };
    let owned_abar_utxo: HashMap<String, u64> = HashMap::new();
    let mut bar_balance = if let Some(val) = owned_utxo.get(FRA_ASSET_CODE) {
        *val
    } else {
        0
    };
    let mut abar_balance = if let Some(val) = owned_abar_utxo.get(FRA_ASSET_CODE) {
        *val
    } else {
        0
    };
    let evm_balance = 0;
    println!(
        "========================================================================================"
    );
    println!("{} FRA", bar_balance + abar_balance + evm_balance);
    println!("- {} FRA(BAR)", bar_balance);
    println!("- {} FRA(ABAR)", abar_balance);
    println!("- {} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
        println!("========================================================================================");
        bar_balance = if let Some(val) = owned_utxo.get(&asset.utxo_asset_code) {
            *val
        } else {
            0
        };
        abar_balance = if let Some(val) = owned_abar_utxo.get(&asset.utxo_asset_code) {
            *val
        } else {
            0
        };
        println!(
            "{} {}",
            bar_balance + abar_balance + evm_balance,
            asset.utxo_symbol
        );
        println!(
            "- {} {: <4}(BAR,{})",
            bar_balance, asset.utxo_symbol, asset.utxo_asset_code
        );
        println!(
            "- {} {: <4}(ABAR,{})",
            abar_balance, asset.utxo_symbol, asset.utxo_asset_code
        );
    }
    Ok(())
}
pub fn show_eth_address(
    chain_net: &ChainNet,
    address: &str,
    kp: &XfrKeyPair,
    mgr: &AssetMgr,
) -> Result<()> {
    let url = format!(
        "{}:{}",
        chain_net.chain_net_address, chain_net.web3_rpc_port
    );
    let evm_addr = if let XfrPublicKeyInner::Secp256k1(pub_key) = kp.pub_key.inner() {
        H160::from(convert_libsecp256k1_public_key_to_address(pub_key))
    } else {
        return Err(anyhow!("evm public key error"));
    };

    let owned_utxo = match get_owned_utxo_balance(&chain_net, &kp) {
        Ok(utxo) => utxo,
        Err(e) => {
            return Err(anyhow!("account {} get_owned_utxos error:{}", address, e));
        }
    };
    let owned_abar_utxo: HashMap<String, u64> = HashMap::new();

    let mut bar_balance = if let Some(val) = owned_utxo.get(FRA_ASSET_CODE) {
        U256::from(*val)
    } else {
        U256::zero()
    };
    let mut abar_balance = if let Some(val) = owned_abar_utxo.get(FRA_ASSET_CODE) {
        U256::from(*val)
    } else {
        U256::zero()
    };
    let mut evm_balance = match get_evm_balance(&url, evm_addr) {
        Ok(balance) => {
            println!("{}", balance);
            balance
        }
        Err(e) => {
            println!("account {} get_evm_balance error:{}", address, e);
            return Ok(());
        }
    };
    println!(
        "========================================================================================"
    );
    println!("{} FRA", bar_balance + abar_balance + evm_balance);
    println!("- {} FRA(BAR)", bar_balance);
    println!("- {} FRA(ABAR)", abar_balance);
    println!("- {} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
        match asset.asset_type {
            Some(AssetType::FRC20) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance = call_erc20_balance_of(url.as_str(), evm_addr, contract_address)?;
                if U256::zero() == bar_balance + abar_balance + evm_balance {
                    continue;
                }
                println!("========================================================================================");
                println!(
                    "{} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(FRC20,{:?})",
                    evm_balance,
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            Some(AssetType::FRC721) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                let tocken_id =
                    match call_erc721_balance_of(url.as_str(), evm_addr, contract_address)? {
                        Some(val) => val,
                        None => continue,
                    };
                println!("========================================================================================");
                println!("{: <4} tocken id:{} ", symbol, tocken_id);
                println!(
                    "- {: >10} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- 1 {: <4}(FRC721,{:?})",
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            Some(AssetType::FRC1155) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                let tocken_id = match asset.tocken_id {
                    Some(val) => val,
                    None => continue,
                };
                evm_balance =
                    call_erc1155_balance_of(url.as_str(), evm_addr, tocken_id, contract_address)?;
                println!("========================================================================================");
                println!(
                    "{} {: <4} tocken id:{}",
                    bar_balance + abar_balance + evm_balance,
                    symbol,
                    tocken_id
                );
                println!(
                    "- {} {: <4}(BAR,{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(FRC1155,{:?})",
                    evm_balance,
                    symbol,
                    asset.contract_address.unwrap_or_default()
                );
            }
            None => {
                bar_balance = if let Some(val) = owned_utxo.get(FRA_ASSET_CODE) {
                    U256::from(*val)
                } else {
                    U256::zero()
                };
                abar_balance = if let Some(val) = owned_abar_utxo.get(FRA_ASSET_CODE) {
                    U256::from(*val)
                } else {
                    U256::zero()
                };
                println!("========================================================================================");
                println!(
                    "{} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    asset.utxo_symbol
                );
                println!(
                    "- {} {: <4}(BAR,{})",
                    bar_balance, asset.utxo_symbol, asset.utxo_asset_code
                );
                println!(
                    "- {} {: <4}(ABAR,{})",
                    abar_balance, asset.utxo_symbol, asset.utxo_asset_code
                );
            }
        }
    }
    Ok(())
}
