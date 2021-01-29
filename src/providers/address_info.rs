use crate::utils::json::default_if_null;
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressInfo {
    pub name: String,
    pub logo_uri: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    pub address: String,
    #[serde(deserialize_with = "default_if_null")]
    pub name: String,
    #[serde(deserialize_with = "default_if_null")]
    pub display_name: String,
    pub logo_uri: Option<String>,
    // pub contract_abi: Option<ContractAbi>, //Ignored for now
}
