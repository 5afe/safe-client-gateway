use super::super::commons::Operation;
use chrono::{DateTime, Utc};
use ethereum_types::{Address, H256};
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
    pub safe: Option<Address>,
    pub to: Address,
    pub value: Option<String>,
    pub data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub operation: Option<Operation>,
    pub gas_token: Option<Address>,
    pub safe_tx_gas: Option<usize>,
    pub base_gas: Option<usize>,
    pub gas_price: Option<String>,
    pub refund_receiver: Option<Address>,
    pub nonce: Option<usize>,
    pub execution_date: Option<DateTime<Utc>>,
    pub submission_date: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub block_number: Option<usize>,
    pub transaction_hash: Option<H256>,
    pub safe_tx_hash: Option<H256>,
    pub executor: Option<Address>,
    pub is_executed: Option<bool>,
    pub is_successful: Option<bool>,
    pub eth_gas_price: Option<String>,
    pub gas_used: Option<usize>,
    pub fee: Option<String>,
    pub origin: Option<Address>,
    pub confirmations_required: Option<usize>,
    pub confirmations: Option<Vec<Confirmation>>,
    pub signatures: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumTransaction {
    pub execution_date: DateTime<Utc>,
    pub to: Address,
    pub data: Option<String>,
    pub tx_hash: H256,
    pub block_number: Option<usize>,
    pub transfers: Option<Vec<Transfer>>,
    pub from: Address,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleTransaction {
    pub created: Option<Address>,
    pub execution_date: Option<DateTime<Utc>>,
    pub block_number: Option<usize>,
    pub transaction_hash: Option<H256>,
    pub safe: Option<Address>,
    pub module: Option<Address>,
    pub to: Address,
    pub value: Option<String>,
    pub data: Option<String>,
    pub operation: Operation,
    // pub transfers: Option<Vec<Transfer>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    owner: Address,
    submission_date: Option<DateTime<Utc>>,
    transaction_hash: Option<H256>,
    confirmation_type: String,
    signature: Option<String>,
}