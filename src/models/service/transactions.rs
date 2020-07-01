use ethereum_types::{Address, H256};
use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::models::commons::DataDecoded;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Transaction {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Unknown,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub sender: Address,
    pub recipient: Address,
    pub date: DateTime<Utc>,
    pub transaction_hash: H256,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferInfo {
    Erc20 {
        token_name: String,
        token_symbol: String,
        logo_uri: String,
        decimals: usize,
        value: String,
    },
    Erc721 {
        token_id: String,
        token_address: Address,
        // logo_uri: String,
    },
    Ether {
        value: String,
    },
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChange {
    pub data_decoded: DataDecoded
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub to: Address
}
