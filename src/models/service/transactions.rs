use serde::Serialize;
use crate::models::commons::DataDecoded;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub timestamp: i64,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub execution_info: Option<ExecutionInfo>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    AwaitingConfirmations,
    AwaitingExecution,
    Cancelled,
    Failed,
    Success,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum TransactionInfo {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Unknown,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionInfo {
    pub nonce: u64,
    pub confirmations_required: u64,
    pub confirmations_submitted: u64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub sender: String,
    pub recipient: String,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferInfo {
    Erc20 {
        token_address: String,
        token_name: Option<String>,
        token_symbol: Option<String>,
        logo_uri: Option<String>,
        decimals: Option<u64>,
        value: String,
    },
    Erc721 {
        token_address: String,
        token_id: String,
        token_name: Option<String>,
        token_symbol: Option<String>,
        logo_uri: Option<String>,
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
    pub to: String,
    pub data_size: String,
    pub value: String,
}
