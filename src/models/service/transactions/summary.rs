use super::*;
use serde::Serialize;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_signers: Option<Vec<String>>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tx_item_type")]
pub enum TransactionListItem {
    #[serde(rename(serialize = "TRANSACTION"))]
    Transaction {
        transaction_summary: TransactionSummary,
        conflict_type: ConflictType,
    },
    #[serde(rename(serialize = "DATE_LABEL"))]
    DateLabel { timestamp: u64 },
    #[serde(rename(serialize = "STRING_LABEL"))]
    StringLabel { label: String },
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ConflictType {
    None,
    Start { nonce: u64 },
    HasNext,
    End,
}
