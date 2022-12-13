use {
    crate::{
        chain_net::ChainNet,
        txn_builder::{TxoSID, Utxo},
    },
    anyhow::{anyhow, Result},
    noah::xfr::{
        asset_record::open_blind_asset_record,
        sig::XfrKeyPair,
        structs::{OpenAssetRecord, OwnerMemo},
    },
    noah_algebra::serialization::NoahFromToBytes,
    std::collections::HashMap,
};

pub fn get_owned_utxo(
    chain_net: &ChainNet,
    kp: &XfrKeyPair,
) -> Result<HashMap<TxoSID, OpenAssetRecord>> {
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
    for (sid, (utxo, owner_memo)) in
        serde_json::from_slice::<HashMap<TxoSID, (Utxo, Option<OwnerMemo>)>>(&bytes)?.iter()
    {
        let open_asset_record = open_blind_asset_record(&utxo.0.record, owner_memo, kp)
            .map_err(|e| anyhow!("open_blind_asset_record error:{:?}", e))?;
        map.insert(sid.clone(), open_asset_record);
    }

    Ok(map)
}
