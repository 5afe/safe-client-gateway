use super::*;
use crate::models::commons::{DataDecoded, Operation};
use crate::providers::info::{SafeAppInfo, TokenInfo};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub executed_at: Option<i64>,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub tx_data: Option<TransactionData>,
    pub detailed_execution_info: Option<DetailedExecutionInfo>,
    pub tx_hash: Option<String>,
    pub safe_app_info: Option<SafeAppInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DetailedExecutionInfo {
    Multisig(MultisigExecutionDetails),
    Module(ModuleExecutionDetails),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigExecutionDetails {
    pub submitted_at: i64,
    pub nonce: u64,
    pub safe_tx_gas: usize,
    pub base_gas: usize,
    pub gas_price: String,
    pub gas_token: String,
    pub refund_receiver: String,
    pub safe_tx_hash: String,
    pub executor: Option<String>,
    pub signers: Vec<String>,
    pub confirmations_required: u64,
    pub confirmations: Vec<MultisigConfirmation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_token_info: Option<TokenInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigConfirmation {
    pub signer: String,
    pub signature: Option<String>,
    pub submitted_at: i64,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleExecutionDetails {
    pub address: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub hex_data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub to: String,
    pub value: Option<String>,
    pub operation: Operation,
}
