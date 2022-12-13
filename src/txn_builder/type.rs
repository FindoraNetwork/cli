use {
    crate::utils::is_default,
    anyhow::{anyhow, Result},
    globutils::{HashOf, SignatureOf},
    noah::xfr::{
        gen_xfr_body,
        sig::XfrKeyPair,
        sig::XfrPublicKey,
        structs::{AssetRecord, TracingPolicies},
        structs::{BlindAssetRecord, XfrBody},
        XfrNotePolicies,
    },
    rand_chacha::rand_core::{CryptoRng, RngCore},
    serde::{Deserialize, Serialize},
    std::hash::{Hash, Hasher},
};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize, Ord, PartialOrd,
)]
pub struct TxoSID(pub u64);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TxOutput {
    pub id: Option<TxoSID>,
    pub record: BlindAssetRecord,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub lien: Option<HashOf<Vec<TxOutput>>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Utxo(pub TxOutput);

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TxoRef {
    Relative(u64),
    Absolute(TxoSID),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TransferType {
    Standard,
    DebtSwap,
}

impl Default for TransferType {
    #[inline(always)]
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransferAssetBody {
    pub inputs: Vec<TxoRef>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub policies: XfrNotePolicies,
    pub outputs: Vec<TxOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub lien_assignments: Vec<(usize, usize, HashOf<Vec<TxOutput>>)>,
    pub transfer: Box<XfrBody>,
    pub transfer_type: TransferType,
}

impl TransferAssetBody {
    pub fn new<R: CryptoRng + RngCore>(
        prng: &mut R,
        input_refs: Vec<TxoRef>,
        input_records: &[AssetRecord],
        output_records: &[AssetRecord],
        policies: Option<XfrNotePolicies>,
        lien_assignments: Vec<(usize, usize, HashOf<Vec<TxOutput>>)>,
        transfer_type: TransferType,
    ) -> Result<TransferAssetBody> {
        let num_inputs = input_records.len();
        let num_outputs = output_records.len();

        if num_inputs == 0 {
            return Err(anyhow!(""));
        }

        // If no policies specified, construct set of empty policies
        let policies = policies.unwrap_or_else(|| {
            let no_policies = TracingPolicies::new();
            XfrNotePolicies::new(
                vec![no_policies.clone(); num_inputs],
                vec![None; num_inputs],
                vec![no_policies; num_outputs],
                vec![None; num_outputs],
            )
        });

        // Verify that for each input and output, there is a corresponding policy and credential commitment
        if num_inputs != policies.inputs_tracing_policies.len()
            || num_inputs != policies.inputs_sig_commitments.len()
            || num_outputs != policies.outputs_tracing_policies.len()
            || num_outputs != policies.outputs_sig_commitments.len()
        {
            return Err(anyhow!(""));
        }

        let transfer = Box::new(
            gen_xfr_body(prng, input_records, output_records).map_err(|e| anyhow!("{}", e))?,
        );
        let outputs = transfer
            .outputs
            .iter()
            .map(|rec| TxOutput {
                id: None,
                record: rec.clone(),
                lien: None,
            })
            .collect();
        Ok(TransferAssetBody {
            inputs: input_refs,
            outputs,
            policies,
            lien_assignments,
            transfer,
            transfer_type,
        })
    }
    pub fn compute_body_signature(
        &self,
        keypair: &XfrKeyPair,
        input_idx: Option<usize>,
    ) -> IndexedSignature<TransferAssetBody> {
        let public_key = keypair.get_pk_ref();
        IndexedSignature {
            signature: SignatureOf::new(keypair, &(self.clone(), input_idx)),
            address: XfrAddress { key: *public_key },
            input_idx,
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct XfrAddress {
    pub key: XfrPublicKey,
}

impl XfrAddress {
    pub(crate) fn to_base64(self) -> String {
        base64::encode_config(&self.key.to_bytes(), base64::URL_SAFE)
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for XfrAddress {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.to_bytes().hash(state);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IndexedSignature<T> {
    pub address: XfrAddress,
    pub signature: SignatureOf<(T, Option<usize>)>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub input_idx: Option<usize>, // Some(idx) if a co-signature, None otherwise
}

impl<T> IndexedSignature<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned,
{
    #[inline(always)]
    #[allow(missing_docs)]
    pub fn verify(&self, message: &T) -> bool {
        self.signature
            .verify(&self.address.key, &(message.clone(), self.input_idx))
            .is_ok()
    }
}
