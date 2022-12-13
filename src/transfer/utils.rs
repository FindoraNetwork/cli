use {
    anyhow::{anyhow, Result},
    bech32::FromBase32,
    noah::xfr::sig::XfrPublicKey,
    noah_algebra::serialization::NoahFromToBytes,
};

pub fn public_key_from_bech32(addr: &str) -> Result<XfrPublicKey> {
    let (hrp, data) = bech32::decode(addr)?;
    let mut d = Vec::<u8>::from_base32(&data)?;
    let bytes = match hrp.as_str() {
        "fra" => {
            let mut a = vec![0u8];
            a.append(&mut d);
            a.append(&mut vec![0u8]);
            a
        }
        "eth" => {
            let mut a = vec![1u8];
            a.append(&mut d);
            a
        }
        _ => {
            return Err(anyhow!("address not supported"));
        }
    };
    XfrPublicKey::noah_from_bytes(&bytes)
        .map_err(|e| anyhow!("XfrPublicKey::noah_from_bytes error:{}", e))
}
