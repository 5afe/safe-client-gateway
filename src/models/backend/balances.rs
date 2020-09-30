use crate::providers::info::TokenInfo;
use serde::Deserialize;

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_address: String,
    pub token: BalanceToken,
    pub balance: String,
    pub balance_usd: String,
    pub usd_conversion: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BalanceToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_uri: String,
}