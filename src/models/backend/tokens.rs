use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceCore {
    pub fiat_code: String,
    pub fiat_price: String,
    pub timestamp: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub address: String,
    pub fiat_code: String,
    pub fiat_price: f64,
    pub timestamp: String,
}
