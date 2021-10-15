use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub fiat_code: String,
    pub fiat_price: BigDecimal,
    pub timestamp: String,
}
