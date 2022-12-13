use {
    crate::chain_net::ChainNet,
    anyhow::{anyhow, Result},
    bech32::FromBase32,
    ethabi::ethereum_types::H160,
    noah::xfr::sig::{XfrKeyPair, XfrPublicKey},
    noah_algebra::serialization::NoahFromToBytes,
};

pub enum Confidential {
    None,
    Amount,
    Asset,
    AmountAsset,
    AmountAssetAddress,
}
pub fn transfer_from_fra_to_fra(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_fra_to_eth(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_fra_to_evm(chain_net: &ChainNet, from: XfrKeyPair, to: H160) {}

pub fn transfer_from_eth_to_fra(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_eth_to_eth(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_eth_to_evm(chain_net: &ChainNet, from: XfrKeyPair, to: H160) {}

pub fn transfer_from_evm_to_fra(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_evm_to_eth(chain_net: &ChainNet, from: XfrKeyPair, to: XfrPublicKey) {}

pub fn transfer_from_evm_to_evm(chain_net: &ChainNet, from: XfrKeyPair, to: H160) {}
