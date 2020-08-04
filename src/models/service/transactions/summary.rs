use serde::Serialize;
use super::*;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSummary {
    pub id: String,
    pub timestamp: i64,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub execution_info: Option<ExecutionInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionInfo {
    pub nonce: u64,
    pub confirmations_required: u64,
    pub confirmations_submitted: u64,
}