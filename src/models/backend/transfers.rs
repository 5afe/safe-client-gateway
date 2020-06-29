use serde::Deserialize;
use chrono::{DateTime, Utc};
use ethereum_types::{Address, U256};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Transfer {
    #[serde(rename(deserialize = "ERC721_TRANSFER"))]
    Erc721(Erc721Transfer),
    #[serde(rename(deserialize = "ERC20_TRANSFER"))]
    Erc20(Erc20Transfer),
    #[serde(rename(deserialize = "ETH_TRANSFER"))]
    Eth(EthTransfer),
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

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthTransfer {
    pub value: U256
}
