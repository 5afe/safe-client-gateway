use crate::providers::info::TokenInfo;
use bigdecimal::BigDecimal;
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
    /// Aggregated fiat balance
    pub fiat_total: String,
    /// Individual [Balance] entries for each ERC20 in the Safe
    pub items: Vec<Balance>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub address: String,
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}
