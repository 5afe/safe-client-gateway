use super::*;
use crate::providers::info::SafeAppInfo;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSummary {
    pub id: String,
    pub timestamp: i64,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub execution_info: Option<ExecutionInfo>,
    pub safe_app_info: Option<SafeAppInfo>,
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
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionListItem {
    #[serde(rename_all = "camelCase")]
    Transaction {
        transaction: TransactionSummary,
        conflict_type: ConflictType,
    },
    DateLabel {
        timestamp: i64,
    },
    Label {
        label: Label,
    },
    ConflictHeader {
        nonce: u64,
    },
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Label {
    Next,
    Queued,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ConflictType {
    None,
    HasNext,
    End,
}
