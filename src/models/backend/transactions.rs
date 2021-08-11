use super::super::commons::Operation;
use crate::models::backend::transfers::Transfer;
use crate::models::commons::DataDecoded;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransaction {
    pub safe: String,
    pub to: String,
    pub value: Option<String>,
    pub data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub operation: Operation,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigTransaction {
    #[serde(flatten)]
    pub safe_transaction: SafeTransaction,
    pub gas_token: Option<String>,
    pub safe_tx_gas: Option<usize>,
    pub base_gas: Option<usize>,
    pub gas_price: Option<String>,
    pub refund_receiver: Option<String>,
    pub nonce: u64,
    pub execution_date: Option<DateTime<Utc>>,
    pub submission_date: DateTime<Utc>,
    pub modified: Option<DateTime<Utc>>,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<String>,
    pub safe_tx_hash: String,
    pub executor: Option<String>,
    pub is_executed: bool,
    pub is_successful: Option<bool>,
    pub eth_gas_price: Option<String>,
    pub gas_used: Option<usize>,
    pub fee: Option<String>,
    pub origin: Option<String>,
    pub confirmations_required: Option<u64>,
    pub confirmations: Option<Vec<Confirmation>>,
    pub signatures: Option<String>,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EthereumTransaction {
    pub execution_date: DateTime<Utc>,
    pub data: Option<String>,
    pub tx_hash: String,
    pub block_number: u64,
    pub transfers: Option<Vec<Transfer>>,
    pub from: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ModuleTransaction {
    #[serde(flatten)]
    pub safe_transaction: SafeTransaction,
    pub created: String,
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub is_successful: bool,
    pub transaction_hash: String,
    pub module: String,
    // pub transfers: Option<Vec<Transfer>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    pub owner: String,
    pub submission_date: DateTime<Utc>,
    pub transaction_hash: Option<String>,
    pub signature_type: String,
    pub signature: Option<String>,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CreationTransaction {
    pub created: DateTime<Utc>,
    pub creator: String,
    pub transaction_hash: String,
    pub factory_address: Option<String>,
    pub master_copy: Option<String>,
    pub setup_data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransactionEstimation {
    pub safe_tx_gas: String,
}
