use super::super::commons::Operation;
use chrono::{DateTime, Utc};
use ethereum_types::{Address, H256};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub safe: Address,
    pub to: Address,
    pub value: String,
    pub data: Option<String>,
    pub operation: Operation,
    pub gas_token: Address,
    pub safe_tx_gas: usize,
    pub base_gas: usize,
    pub gas_price: String,
    pub refund_receiver: Address,
    pub nonce: usize,
    pub execution_date: Option<DateTime<Utc>>,
    pub submission_date: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub block_number: Option<usize>,
    pub transaction_hash: Option<H256>,
    pub safe_tx_hash: H256,
    pub executor: Option<Address>,
    pub is_executed: bool,
    pub is_successful: Option<bool>,
    pub eth_gas_price: Option<String>,
    pub gas_used: Option<usize>,
    pub fee: Option<String>,
    pub origin: Option<Address>,
    pub confirmations_required: Option<usize>,
    pub confirmations: Vec<Confirmation>,
    pub signatures: Option<String>,
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