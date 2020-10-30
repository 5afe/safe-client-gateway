use crate::providers::info::TokenInfo;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_info: TokenInfo,
    pub balance: String,
    pub fiat_balance: String,
    pub fiat_conversion: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    pub fiat_total: String,
    pub items: Vec<Balance>,
}
