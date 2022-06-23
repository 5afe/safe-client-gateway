use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct Delegate {
    safe: Option<String>,
    delegate: String,
    delegator: String,
    label: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateCreate {
    safe: Option<String>,
    delegate: String,
    delegator: String,
    signature: String,
    label: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateDelete {
    delegate: String,
    delegator: String,
    signature: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeDelegateDelete {
    safe: String,
    delegate: String,
    signature: String,
}
