use super::super::commons::Operation;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::DataDecoded;

#[derive(Deserialize, Debug)]
#[serde(tag = "txType")]
pub enum Transaction {
    #[serde(rename(deserialize = "MULTISIG_TRANSACTION"))]
    Multisig(MultisigTransaction),
    #[serde(rename(deserialize = "ETHEREUM_TRANSACTION"))]
    Ethereum(EthereumTransaction),
    #[serde(rename(deserialize = "MODULE_TRANSACTION"))]
    Module(ModuleTransaction),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultisigTransaction {
    pub safe: String,
    pub to: String,
    pub value: Option<String>,
    pub data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub operation: Option<Operation>,
    pub gas_token: Option<String>,
    pub safe_tx_gas: Option<usize>,
    pub base_gas: Option<usize>,
    pub gas_price: Option<String>,
    pub refund_receiver: Option<String>,
    pub nonce: u64,
    pub execution_date: Option<DateTime<Utc>>,
    pub submission_date: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
    pub block_number: Option<usize>,
    pub transaction_hash: Option<String>,
    pub safe_tx_hash: Option<String>,
    pub executor: Option<String>,
    pub is_executed: bool,
    pub is_successful: Option<bool>,
    pub eth_gas_price: Option<String>,
    pub gas_used: Option<usize>,
    pub fee: Option<String>,
    pub origin: Option<String>,
    pub confirmations_required: Option<usize>,
    pub confirmations: Option<Vec<Confirmation>>,
    pub signatures: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumTransaction {
    pub execution_date: DateTime<Utc>,
    pub to: String,
    pub data: Option<String>,
    pub tx_hash: String,
    pub block_number: Option<usize>,
    pub transfers: Option<Vec<Transfer>>,
    pub from: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleTransaction {
    pub created: Option<String>,
    pub execution_date: DateTime<Utc>,
    pub block_number: Option<usize>,
    pub transaction_hash: Option<String>,
    pub safe: Option<String>,
    pub module: Option<String>,
    pub to: String,
    pub value: Option<String>,
    pub data: Option<String>,
    pub operation: Operation,
    // pub transfers: Option<Vec<Transfer>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    owner: String,
    submission_date: Option<DateTime<Utc>>,
    transaction_hash: Option<String>,
    confirmation_type: String,
    signature: Option<String>,
}