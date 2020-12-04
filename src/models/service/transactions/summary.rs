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
    DateLabel { timestamp: i64 },
    #[serde(rename(serialize = "LABEL"))]
    Label { label: Label },
    #[serde(rename(serialize = "CONFLICT_HEADER"))]
    ConflictHeader { nonce: u64 },
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Label {
    Next,
    Queued,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConflictType {
    None,
    HasNext,
    End,
}
