use crate::providers::info::TokenInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_info: TokenInfo,
    pub amount: String,
    pub fiat_amount: String,
}