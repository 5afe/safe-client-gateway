use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfoEx {
    pub address: AddressEx,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<AddressEx>,
    pub implementation: AddressEx,
    pub modules: Option<Vec<AddressEx>>,
    pub fallback_handler: Option<AddressEx>,
    pub version: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddressEx {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
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
