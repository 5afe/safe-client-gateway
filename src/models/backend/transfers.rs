use crate::providers::info::TokenInfo;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Hash)]
#[serde(tag = "type")]
pub enum Transfer {
    #[serde(rename(deserialize = "ERC721_TRANSFER"))]
    Erc721(Erc721Transfer),
    #[serde(rename(deserialize = "ERC20_TRANSFER"))]
    Erc20(Erc20Transfer),
    #[serde(rename(deserialize = "ETHER_TRANSFER"))]
    Ether(EtherTransfer),
    #[serde(other)]
    Unknown,
}

#[derive(Derivative, Deserialize, Debug, PartialEq, Clone)]
#[derivative(Hash)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: String,
    pub to: String,
    pub token_id: String,
    pub token_address: String,
    #[derivative(Hash = "ignore")]
    pub token_info: Option<TokenInfo>,
    pub from: String,
}

#[derive(Derivative, Deserialize, Debug, Clone)]
#[derivative(Hash)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: String,
    pub to: String,
    pub value: String,
    pub token_address: String,
    #[derivative(Hash = "ignore")]
    pub token_info: Option<TokenInfo>,
    pub from: String,
}

#[derive(Derivative, Deserialize, Debug, Clone)]
#[derivative(Hash)]
#[serde(rename_all = "camelCase")]
pub struct EtherTransfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: String,
    pub to: String,
    pub value: String,
    pub from: String,
}
