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
    pub to: Address,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub token_address: Address,
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub to: Address,

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
