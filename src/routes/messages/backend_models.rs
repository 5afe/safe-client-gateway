use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum SignatureType {
    ContractSignature,
    ApprovedHash,
    Eoa,
    EthSign,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Confirmation {
    pub(super) created: DateTime<Utc>,
    pub(super) modified: DateTime<Utc>,
    pub(super) owner: String,
    pub(super) signature: String,
    pub(super) signature_type: SignatureType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Message {
    pub(super) created: DateTime<Utc>,
    pub(super) modified: DateTime<Utc>,
    pub(super) safe: String,
    pub(super) message_hash: String,
    pub(super) message: String,
    pub(super) proposed_by: String,
    pub(super) safe_app_id: Option<u64>,
    pub(super) confirmations: Vec<Confirmation>,
    pub(super) prepared_signature: Option<String>,
}
