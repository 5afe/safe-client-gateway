use super::addresses::AddressEx;
use serde::Serialize;

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
