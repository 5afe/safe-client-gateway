use crate::providers::info::TokenInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum AddressInfo {
    TokenInfo(TokenInfo),
    KnownAddress(KnownAddress),
}

#[derive(Serialize, Debug)]
pub struct KnownAddress {
    pub name: String,
    pub logo_uri: String,
}
