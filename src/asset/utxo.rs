use {
    crate::chain_net::ChainNet,
    anyhow::{anyhow, Result},
    noah::xfr::{
        asset_record::open_blind_asset_record,
        sig::XfrKeyPair,
        structs::{BlindAssetRecord, OwnerMemo},
    },
    noah_algebra::serialization::NoahFromToBytes,
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct TxoSID(pub u64);

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub id: Option<TxoSID>,
    pub record: BlindAssetRecord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Utxo(pub TxOutput);

pub fn get_owned_utxo_balance(
    chain_net: &ChainNet,
    kp: &XfrKeyPair,
) -> Result<HashMap<String, u64>> {
    let url = format!(
        "{}:{}/owned_utxos/{}",
        chain_net.chain_net_address,
        chain_net.query_port,
        base64::encode_config(
            &NoahFromToBytes::noah_to_bytes(&kp.pub_key),
            base64::URL_SAFE
        )
    );

    let mut map = HashMap::new();
    let bytes = attohttpc::get(&url).send()?.error_for_status()?.bytes()?;
    for (_, (utxo, owner_memo)) in
        serde_json::from_slice::<HashMap<TxoSID, (Utxo, Option<OwnerMemo>)>>(&bytes)?.iter()
    {
        let open_asset_record = open_blind_asset_record(&utxo.0.record, owner_memo, kp)
            .map_err(|e| anyhow!("open_blind_asset_record error:{:?}", e))?;
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
