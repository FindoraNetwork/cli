use {
    crate::txn_builder::{IndexedSignature, TransferAssetBody},
    anyhow::{anyhow, Result},
    noah::xfr::sig::XfrKeyPair,
    serde::{Deserialize, Serialize},
};

/// Operation list supported in findora network
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Operation {
    TransferAsset(TransferAsset),
    IssueAsset,
    DefineAsset,
    UpdateMemo,
    UpdateStaker,
    Delegation,
    UnDelegation,
    Claim,
    UpdateValidator,
    Governance,
    FraDistribution,
    MintFra,
    ConvertAccount,
    BarToAbar,
    AbarToBar,
    TransferAnonAsset,
    ReplaceStaker,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransferAsset {
    pub body: TransferAssetBody,
    pub body_signatures: Vec<IndexedSignature<TransferAssetBody>>,
}
impl TransferAsset {
    pub fn new(transfer_body: TransferAssetBody) -> Self {
        Self {
            body: transfer_body,
            body_signatures: Vec::new(),
        }
    }
    pub fn sign(&mut self, keypair: &XfrKeyPair) {
        let sig = self.create_input_signature(keypair);
        self.attach_signature(sig).unwrap()
    }
    #[inline(always)]
    #[allow(missing_docs)]
    pub fn attach_signature(&mut self, sig: IndexedSignature<TransferAssetBody>) -> Result<()> {
        if !sig.verify(&self.body) {
            return Err(anyhow!(""));
        }
        self.body_signatures.push(sig);
        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn create_input_signature(
        &self,
        keypair: &XfrKeyPair,
    ) -> IndexedSignature<TransferAssetBody> {
        self.body.compute_body_signature(keypair, None)
    }
}
