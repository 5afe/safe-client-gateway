use serde::Deserialize;
use chrono::{DateTime, Utc};
use ethereum_types::{Address, H256};

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub to: Address,
    pub token_id: String,
    pub token_address: Address,
    pub from: Address,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub to: Address,
    pub value: String,
    pub token_address: Address,
    pub token_info: Erc20TokenInfo,
    pub from: Address,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc20TokenInfo {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: usize,
    pub logo_uri: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EtherTransfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub to: Address,
    pub value: String,
    pub from: Address,
}
