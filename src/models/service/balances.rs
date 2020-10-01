use crate::providers::info::{TokenInfo, DefaultInfoProvider};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub token_info: TokenInfo,
    pub amount: String,
    pub fiat_amount: String,
}

impl Fiat {
    pub fn from_dasd(fiat: &str, info_provider: &DefaultInfoProvider) -> Self {
        match fiat {
            "usd" => Fiat::Usd(1.0),
            "eur" => Fiat::Eur(info_provider.exchange_usd_to_eur().unwrap_or(1.0)),
            _ => Fiat::Unknown
        }
    }
}

pub enum Fiat {
    Usd(f64),
    Eur(f64),
    Unknown,
}
