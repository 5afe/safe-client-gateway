use super::super::commons::{Operation, TransactionType};
use chrono::{DateTime, Utc};
use ethereum_types::{Address, H256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub safe: Option<Address>,
    pub to: Address,
    pub value: Option<String>,
    pub data: Option<String>,
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
    pub tx_type: TransactionType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    owner: Address,
    submission_date: Option<DateTime<Utc>>,
    transaction_hash: Option<H256>,
    confirmation_type: String,
    // looks like it should be an enum
    signature: Option<String>,
}