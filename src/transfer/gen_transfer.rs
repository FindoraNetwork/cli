use {
    crate::{
        chain_net::ChainNet,
        txn_builder::{Operation, TransferOperationBuilder, TransferType},
        utils::get_owned_utxo,
    },
    anyhow::{anyhow, Result},
    ed25519_dalek_bip32::ed25519_dalek,
    lazy_static::lazy_static,
    noah::xfr::{
        asset_record::AssetRecordType,
        sig::{XfrKeyPair, XfrPublicKey},
        structs::{AssetRecordTemplate, AssetType, ASSET_TYPE_LENGTH},
    },
    noah_algebra::serialization::NoahFromToBytes,
};

lazy_static! {
    pub static ref BLACK_HOLE_PUBKEY: XfrPublicKey =
        XfrPublicKey::noah_from_bytes(&[0; ed25519_dalek::PUBLIC_KEY_LENGTH][..]).unwrap();
}

const TX_FEE_MIN: u64 = 1_0000;
pub const ASSET_TYPE_FRA: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);

pub fn gen_transfer_op(
    chain_net: &ChainNet,
    owner_kp: &XfrKeyPair,
    mut target_list: Vec<(&XfrPublicKey, u64)>,
    token_code: Option<AssetType>,
    asset_record_type: AssetRecordType,
) -> Result<Operation> {
    let asset_type = token_code.unwrap_or(ASSET_TYPE_FRA);

    let mut am: u64 = target_list.iter().map(|(_, am)| *am).sum();
    let mut op_fee: u64 = if asset_type == ASSET_TYPE_FRA {
        am += TX_FEE_MIN;
        TX_FEE_MIN
    } else {
        0
    };
    let mut trans_builder = TransferOperationBuilder::new();

    let mut i_am = 0;
    for (sid, open_asset_record) in get_owned_utxo(chain_net, owner_kp)? {
        if open_asset_record.asset_type == ASSET_TYPE_FRA && op_fee != 0 {
            if open_asset_record.amount < op_fee {
                i_am = open_asset_record.amount
            } else {
                i_am = op_fee
            }
            // trans_builder.add_input(TxoRef::Absolute(sid), open_asset_record, None, None, i_am)?;
            op_fee -= i_am;
        } else if am != 0 {
            if open_asset_record.amount < am {
                i_am = open_asset_record.amount
            } else {
                i_am = am
            }
            // trans_builder.add_input(TxoRef::Absolute(sid), open_asset_record, None, None, i_am)?;
            am -= i_am;
        }
        if 0 == am && 0 == op_fee {
            break;
        }
    }

    if 0 != am || 0 != op_fee {
        return Err(anyhow!("insufficient balance"));
    }

    trans_builder
        .add_output(
            &AssetRecordTemplate::with_no_asset_tracing(
                TX_FEE_MIN,
                ASSET_TYPE_FRA,
                AssetRecordType::NonConfidentialAmount_NonConfidentialAssetType,
                *BLACK_HOLE_PUBKEY,
            ),
            None,
            None,
            None,
        )
        .map_err(|e| anyhow!("{}", e))?;
    for (pk, am) in target_list {
        trans_builder
            .add_output(
                &AssetRecordTemplate::with_no_asset_tracing(am, asset_type, asset_record_type, *pk),
                None,
                None,
                None,
            )
            .map_err(|e| anyhow!("{}", e))?;
    }

    trans_builder
        .balance()?
        .create(TransferType::Standard)?
        .sign(owner_kp)?
        .transaction()
}
