use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_address: Option<String>,
    pub token: Option<BalanceToken>,
    pub balance: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BalanceToken {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_uri: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}
