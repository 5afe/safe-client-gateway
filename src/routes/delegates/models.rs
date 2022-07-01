use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DelegateCreate {
    safe: Option<String>,
    delegate: String,
    delegator: String,
    signature: String,
    label: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DelegateDelete {
    delegate: String,
    delegator: String,
    signature: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SafeDelegateDelete {
    safe: String,
    delegate: String,
    signature: String,
}
