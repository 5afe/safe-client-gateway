use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceCore {
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub address: String,
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}
