use crate::utils::json::default_if_null;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct ContractInfo {
    pub address: String,
    #[serde(deserialize_with = "default_if_null")]
    pub name: String,
    #[serde(deserialize_with = "default_if_null")]
    pub display_name: String,
    pub logo_uri: Option<String>,
    pub contract_abi: Option<Value>,
    pub trusted_for_delegate_call: Option<bool>,
}
