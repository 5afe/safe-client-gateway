use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub address: String,
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}
