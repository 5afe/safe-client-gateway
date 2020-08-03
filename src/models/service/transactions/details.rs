use serde::Serialize;
use super::*;
use crate::models::commons::{Operation, DataDecoded};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub executed_at: Option<i64>,
    pub submitted_at: Option<i64>,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub tx_data: Option<TransactionData>,
    pub detailed_execution_info: Option<DetailedExecutionInfo>,
    pub tx_hash: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetailedExecutionInfo {
    pub nonce: u64,
    pub operation: Operation,
    pub safe_tx_hash: String,
    pub signers: Vec<String>,
    pub confirmations_required: u64,
    pub confirmations: Vec<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub hex_data: String,
    pub data_decoded: Option<DataDecoded>,
}
