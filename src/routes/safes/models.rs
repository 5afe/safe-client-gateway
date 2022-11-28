use crate::common::models::addresses::AddressEx;
use crate::common::models::data_decoded::Operation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Deserialize))]
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
#[cfg_attr(test, derive(Deserialize))]
pub struct SafeLastChanges {
    pub collectibles_tag: String,
    pub tx_queued_tag: String,
    pub tx_history_tag: String,
    // Can be String once the Messages feature is considered stable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_tag: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Deserialize))]
pub struct SafeState {
    #[serde(flatten)]
    pub safe_config: SafeInfoEx,
    #[serde(flatten)]
    pub safe_state: SafeLastChanges,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(Deserialize))]
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
#[cfg_attr(test, derive(Deserialize, PartialEq))]
pub struct SafeTransactionEstimation {
    pub current_nonce: u64,
    pub latest_nonce: u64,
    pub safe_tx_gas: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Deserialize, PartialEq))]
pub struct SafeTransactionEstimationV2 {
    pub current_nonce: u64,
    pub recommended_nonce: u64,
    pub safe_tx_gas: String,
}
