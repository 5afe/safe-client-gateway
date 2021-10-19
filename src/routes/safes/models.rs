use crate::common::models::service::addresses::AddressEx;
use crate::common::models::Operation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfoEx {
    pub address: AddressEx,
    pub chain_id: String,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<AddressEx>,
    pub implementation: AddressEx,
    pub modules: Option<Vec<AddressEx>>,
    pub fallback_handler: Option<AddressEx>,
    pub guard: Option<AddressEx>,
    pub version: Option<String>,
    pub implementation_version_state: ImplementationVersionState,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeLastChanges {
    pub collectibles_tag: String,
    pub tx_queued_tag: String,
    pub tx_history_tag: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeState {
    #[serde(flatten)]
    pub safe_config: SafeInfoEx,
    #[serde(flatten)]
    pub safe_state: SafeLastChanges,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImplementationVersionState {
    UpToDate,
    Outdated,
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Implementation {
    pub address: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SafeTransactionEstimationRequest {
    // Address will not be mapped to AddressEx as it is a POST body that is forwarded to the core services
    pub to: String,
    pub value: String,
    pub data: String,
    pub operation: Operation,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransactionEstimation {
    pub latest_nonce: u64,
    pub safe_tx_gas: String,
}
