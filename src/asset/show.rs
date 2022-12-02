use {
    super::{AssetMgr, AssetType},
    crate::{
        asset::{
            call_erc1155_balance_of, call_erc20_balance_of, call_erc721_balance_of,
            get_evm_balance, get_owned_utxo_balance,
        },
        server::Server,
    },
    anyhow::{anyhow, Result},
    ethabi::ethereum_types::H160,
    noah::xfr::sig::{convert_libsecp256k1_public_key_to_address, XfrKeyPair, XfrPublicKeyInner},
    std::{collections::HashMap, str::FromStr},
};

const FRA_ASSET_CODE: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

pub fn show_evm_address(server: &Server, address: &str, mgr: &AssetMgr) -> Result<()> {
    let url = format!("{}:{}", server.server_address, server.web3_rpc_port);
    let address = H160::from_str(address)?;
    let bar_balance = 0;
    let abar_balance = 0;
    let mut evm_balance = match get_evm_balance(&url, address) {
        Ok(balance) => balance.as_u64(),
        Err(e) => {
            println!("account {} get_evm_balance error:{}", address, e);
            return Ok(());
        }
    };
    println!(
        "========================================================================================"
    );
    println!("{: <12} FRA", bar_balance + abar_balance + evm_balance,);
    println!("- {: <12} FRA(BAR)", bar_balance);
    println!("- {: <12} FRA(ABAR)", abar_balance);
    println!("- {: <12} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
        match asset.asset_type {
            Some(AssetType::FRC20) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance =
                    call_erc20_balance_of(url.as_str(), address, contract_address)?.as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC20,\t{:?})",
                    evm_balance, symbol, asset.contract_address
                );
            }
            Some(AssetType::FRC721) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance =
                    call_erc721_balance_of(url.as_str(), address, contract_address)?.as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC721,\t{:?})",
                    evm_balance, symbol, asset.contract_address
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
                    call_erc1155_balance_of(url.as_str(), address, tocken_id, contract_address)?
                        .as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4} {}",
                    bar_balance + abar_balance + evm_balance,
                    symbol,
                    tocken_id
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC1155,\t{:?})",
                    evm_balance, symbol, asset.contract_address
                );
            }
            _ => {}
        };
    }
    Ok(())
}
pub fn show_fra_address(
    server: &Server,
    address: &str,
    kp: &XfrKeyPair,
    mgr: &AssetMgr,
) -> Result<()> {
    let owned_utxo = match get_owned_utxo_balance(&server, &kp) {
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
    println!("{: <12} FRA", bar_balance + abar_balance + evm_balance);
    println!("- {: <12} FRA(BAR)", bar_balance);
    println!("- {: <12} FRA(ABAR)", abar_balance);
    println!("- {: <12} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
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
        println!("========================================================================================");
        println!(
            "{: <12} {}",
            bar_balance + abar_balance + evm_balance,
            asset.utxo_symbol
        );
        println!(
            "- {: <12} {: <4}(BAR,\t{})",
            bar_balance, asset.utxo_symbol, asset.utxo_asset_code
        );
        println!(
            "- {: <12} {: <4}(ABAR,\t{})",
            abar_balance, asset.utxo_symbol, asset.utxo_asset_code
        );
    }
    Ok(())
}
pub fn show_eth_address(
    server: &Server,
    address: &str,
    kp: &XfrKeyPair,
    mgr: &AssetMgr,
) -> Result<()> {
    let url = format!("{}:{}", server.server_address, server.web3_rpc_port);
    let evm_addr = if let XfrPublicKeyInner::Secp256k1(pub_key) = kp.pub_key.inner() {
        H160::from(convert_libsecp256k1_public_key_to_address(pub_key))
    } else {
        return Err(anyhow!("evm public key error"));
    };

    let owned_utxo = match get_owned_utxo_balance(&server, &kp) {
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
    let mut evm_balance = match get_evm_balance(&url, evm_addr) {
        Ok(balance) => balance.as_u64(),
        Err(e) => {
            println!("account {} get_evm_balance error:{}", address, e);
            return Ok(());
        }
    };
    println!(
        "========================================================================================"
    );
    println!("{: <12} FRA", bar_balance + abar_balance + evm_balance);
    println!("- {: <12} FRA(BAR)", bar_balance);
    println!("- {: <12} FRA(ABAR)", abar_balance);
    println!("- {: <12} FRA(EVM)", evm_balance);

    for asset in mgr.assets.values() {
        match asset.asset_type {
            Some(AssetType::FRC20) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance =
                    call_erc20_balance_of(url.as_str(), evm_addr, contract_address)?.as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC20,\t{:?})",
                    evm_balance, symbol, asset.contract_address
                );
            }
            Some(AssetType::FRC721) => {
                let contract_address = match asset.contract_address {
                    Some(val) => val,
                    None => continue,
                };
                let symbol = asset.symbol.as_deref().unwrap_or("");
                evm_balance =
                    call_erc721_balance_of(url.as_str(), evm_addr, contract_address)?.as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    symbol
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC721,\t{:?})",
                    evm_balance, symbol, asset.contract_address
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
                    call_erc1155_balance_of(url.as_str(), evm_addr, tocken_id, contract_address)?
                        .as_u64();
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4} {}",
                    bar_balance + abar_balance + evm_balance,
                    symbol,
                    tocken_id
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(FRC1155,\t{:?})",
                    evm_balance, symbol, asset.contract_address
                );
            }
            None => {
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
                println!("========================================================================================");
                println!(
                    "{: <12} {: <4}",
                    bar_balance + abar_balance + evm_balance,
                    asset.utxo_symbol
                );
                println!(
                    "- {: <12} {: <4}(BAR,\t{})",
                    bar_balance, asset.utxo_symbol, asset.utxo_asset_code
                );
                println!(
                    "- {: <12} {: <4}(ABAR,\t{})",
                    abar_balance, asset.utxo_symbol, asset.utxo_asset_code
                );
            }
        }
    }
    Ok(())
}
