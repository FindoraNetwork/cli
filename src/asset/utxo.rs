use {
    crate::server::Server,
    anyhow::Result,
    noah::xfr::{
        sig::XfrPublicKey,
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
    server: &Server,
    addr: &XfrPublicKey,
) -> Result<HashMap<String, u64>> {
    let url = format!(
        "{}:{}/owned_utxos/{}",
        server.server_address,
        server.query_port,
        base64::encode_config(&NoahFromToBytes::noah_to_bytes(addr), base64::URL_SAFE)
    );

    let mut map = HashMap::new();
    let bytes = attohttpc::get(&url).send()?.error_for_status()?.bytes()?;
    for (_, (utxo, _)) in
        serde_json::from_slice::<HashMap<TxoSID, (Utxo, Option<OwnerMemo>)>>(&bytes)?.iter()
    {
        let asset = match utxo.0.record.asset_type {
            noah::xfr::structs::XfrAssetType::Confidential(_) => continue,
            noah::xfr::structs::XfrAssetType::NonConfidential(input) => {
                base64::encode_config(input.0, base64::URL_SAFE)
            }
        };
        let amount = match utxo.0.record.amount {
            noah::xfr::structs::XfrAmount::Confidential(_) => continue,
            noah::xfr::structs::XfrAmount::NonConfidential(val) => val,
        };
        match map.get(&asset) {
            Some(val) => {
                let val = (*val) + amount;
                map.insert(asset, val);
            }
            None => {
                map.insert(asset, amount);
            }
        };
    }

    Ok(map)
}
