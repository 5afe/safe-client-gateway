use crate::providers::info::TokenInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_info: TokenInfo,
    pub amount: String,
    pub fiat_amount: String,
}

impl From<String> for Fiat {
    fn from(fiat: String) -> Self {
        match fiat.as_str() {
            "usd" => Fiat::Usd,
            "eur" => Fiat::Eur,
            _ => Fiat::Unknown
        }
    }
}

pub enum Fiat {
    Usd,
    Eur,
    Unknown,
}
