use {
    crate::{chain_net::ChainNet, utils::get_owned_utxo},
    anyhow::Result,
    noah::xfr::sig::XfrKeyPair,
    std::collections::HashMap,
};

pub(super) fn get_owned_utxo_balance(
    chain_net: &ChainNet,
    kp: &XfrKeyPair,
) -> Result<HashMap<String, u64>> {
    let mut map = HashMap::new();
    for (_, open_asset_record) in get_owned_utxo(chain_net, kp)? {
        let asset = format!("0x{}", hex::encode(open_asset_record.asset_type.0));
        match map.get(&asset) {
            Some(val) => {
                let val = (*val) + open_asset_record.amount;
                map.insert(asset, val);
            }
            None => {
                map.insert(asset, open_asset_record.amount);
            }
        };
    }
    Ok(map)
}
