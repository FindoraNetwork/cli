use {
    super::{Operation, TransferAsset, TransferAssetBody, TransferType, TxoRef},
    anyhow::{anyhow, Result},
    credentials::CredUserSecretKey,
    noah::{
        anon_creds::{ACCommitment, ACCommitmentKey, Credential},
        xfr::{
            sig::XfrKeyPair,
            structs::{AssetRecord, AssetRecordTemplate, OpenAssetRecord, TracingPolicies},
            XfrNotePolicies,
        },
    },
    noah_algebra::prelude::*,
    rand_chacha::ChaChaRng,
    serde::{Deserialize, Serialize},
    std::cmp::Ordering,
};

macro_rules! no_transfer_err {
    () => {
        ("Transaction has not yet been finalized".to_string())
    };
}
/// TransferOperationBuilder constructs transfer operations using the factory pattern
/// Inputs and outputs are added iteratively before being signed by all input record owners
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TransferOperationBuilder {
    input_sids: Vec<TxoRef>,
    spend_amounts: Vec<u64>, // Amount of each input record to spend, the rest will be refunded if user calls balance
    input_records: Vec<AssetRecord>,
    inputs_tracing_policies: Vec<TracingPolicies>,
    input_identity_commitments: Vec<Option<ACCommitment>>,
    output_records: Vec<AssetRecord>,
    outputs_tracing_policies: Vec<TracingPolicies>,
    output_identity_commitments: Vec<Option<ACCommitment>>,
    transfer: Option<TransferAsset>,
    transfer_type: TransferType,
    auto_refund: bool,
}

impl TransferOperationBuilder {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            auto_refund: true,
            ..Default::default()
        }
    }

    pub fn add_input(
        &mut self,
        txo_sid: TxoRef,
        open_ar: OpenAssetRecord,
        tracing_policies: Option<TracingPolicies>,
        identity_commitment: Option<ACCommitment>,
        amount: u64,
    ) -> Result<&mut Self> {
        if self.transfer.is_some() {
            return Err(anyhow!("Cannot mutate a transfer that has been signed"));
        }
        let policies = tracing_policies.unwrap_or_default();

        let asset_record = AssetRecord::from_open_asset_record_with_asset_tracing_but_no_identity(
            &mut ChaChaRng::from_entropy(),
            open_ar,
            policies.clone(),
        )
        .map_err(|e| anyhow!("{}", e))?;
        self.input_sids.push(txo_sid);
        self.input_records.push(asset_record);
        self.inputs_tracing_policies.push(policies);
        self.input_identity_commitments.push(identity_commitment);
        self.spend_amounts.push(amount);
        Ok(self)
    }

    #[allow(missing_docs)]
    pub fn add_output(
        &mut self,
        asset_record_template: &AssetRecordTemplate,
        tracing_policies: Option<TracingPolicies>,
        identity_commitment: Option<ACCommitment>,
        credential_record: Option<(&CredUserSecretKey, &Credential, &ACCommitmentKey)>,
    ) -> Result<&mut Self> {
        let prng = &mut ChaChaRng::from_entropy();
        if self.transfer.is_some() {
            return Err(anyhow!("Cannot mutate a transfer that has been signed"));
        }
        let policies = tracing_policies.unwrap_or_default();
        let ar = if let Some((user_secret_key, credential, commitment_key)) = credential_record {
            AssetRecord::from_template_with_identity_tracing(
                prng,
                asset_record_template,
                user_secret_key.get_ref(),
                credential,
                commitment_key,
            )
            .map_err(|e| anyhow!("{}", e))?
        } else {
            AssetRecord::from_template_no_identity_tracing(prng, asset_record_template)
                .map_err(|e| anyhow!("{}", e))?
        };
        self.output_records.push(ar);
        self.outputs_tracing_policies.push(policies);
        self.output_identity_commitments.push(identity_commitment);
        Ok(self)
    }

    fn check_balance(&self) -> Result<()> {
        let input_total: u64 = self
            .input_records
            .iter()
            .fold(0, |acc, ar| acc + ar.open_asset_record.amount);
        let output_total = self
            .output_records
            .iter()
            .fold(0, |acc, ar| acc + ar.open_asset_record.amount);
        if input_total != output_total {
            return Err(anyhow!(format!("{} != {}", input_total, output_total)));
        }

        Ok(())
    }

    pub fn balance(&mut self) -> Result<&mut Self> {
        let mut prng = ChaChaRng::from_entropy();
        if self.transfer.is_some() {
            return Err(anyhow!("Cannot mutate a transfer that has been signed"));
        }
        let mut amt_cache = vec![];

        let spend_total: u64 = self.spend_amounts.iter().sum();
        let mut partially_consumed_inputs = Vec::new();

        for (idx, ((spend_amount, ar), policies)) in self
            .spend_amounts
            .iter()
            .zip(self.input_records.iter())
            .zip(self.inputs_tracing_policies.iter())
            .enumerate()
        {
            let amt = ar.open_asset_record.get_amount();
            match spend_amount.cmp(amt) {
                Ordering::Greater => {
                    return Err(anyhow!(""));
                }
                Ordering::Less => {
                    let asset_type = *ar.open_asset_record.get_asset_type();
                    let record_type = ar.open_asset_record.get_record_type();
                    let recipient = *ar.open_asset_record.get_pub_key();
                    let ar_template = AssetRecordTemplate::with_asset_tracing(
                        amt - spend_amount,
                        asset_type,
                        record_type,
                        recipient,
                        policies.clone(),
                    );
                    let ar =
                        AssetRecord::from_template_no_identity_tracing(&mut prng, &ar_template)
                            .map_err(|e| anyhow!("{}", e))?;
                    partially_consumed_inputs.push(ar);
                    self.outputs_tracing_policies.push(policies.clone());
                    self.output_identity_commitments.push(None);

                    amt_cache.push((idx, *amt));
                }
                _ => {}
            }
        }

        let output_total = self
            .output_records
            .iter()
            .fold(0, |acc, ar| acc + ar.open_asset_record.amount);
        if spend_total != output_total {
            return Err(anyhow!(
                "Spend total != output, {} != {}",
                spend_total,
                output_total
            ));
        }
        self.output_records.append(&mut partially_consumed_inputs);

        amt_cache.into_iter().for_each(|(idx, am)| {
            self.spend_amounts[idx] = am;
        });

        Ok(self)
    }

    pub fn create(&mut self, transfer_type: TransferType) -> Result<&mut Self> {
        if self.auto_refund {
            self.balance().map_err(|e| anyhow!("{}", e))?;
        } else {
            self.check_balance().map_err(|e| anyhow!("{}", e))?;
        }

        let mut prng = ChaChaRng::from_entropy();
        let num_inputs = self.input_records.len();
        let num_outputs = self.output_records.len();
        let xfr_policies = XfrNotePolicies::new(
            self.inputs_tracing_policies.clone(),
            vec![None; num_inputs],
            self.outputs_tracing_policies.clone(),
            vec![None; num_outputs],
        );
        let body = TransferAssetBody::new(
            &mut prng,
            self.input_sids.clone(),
            &self.input_records,
            &self.output_records,
            Some(xfr_policies),
            vec![],
            transfer_type,
        )
        .map_err(|e| anyhow!("{}", e))?;
        self.transfer = Some(TransferAsset::new(body));
        Ok(self)
    }

    pub fn sign(&mut self, kp: &XfrKeyPair) -> Result<&mut Self> {
        if self.transfer.is_none() {
            return Err(anyhow!(no_transfer_err!()));
        }
        self.transfer.as_mut().ok_or(anyhow!(""))?.sign(&kp);
        Ok(self)
    }

    /// Return the transaction operation
    pub fn transaction(&self) -> Result<Operation> {
        if self.transfer.is_none() {
            return Err(anyhow!(no_transfer_err!()));
        }
        Ok(Operation::TransferAsset(
            self.transfer.clone().ok_or(anyhow!(""))?,
        ))
    }
}
