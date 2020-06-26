use ethereum_types::{Address, H256, U256};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::super::commons::TransactionType;
use crate::models::commons::ServiceTransactionType;

#[typetag::serde(tag = "transaction_type")]
pub trait ServiceTransaction {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transfer {
    pub to: Address,
    pub transaction_type: ServiceTransactionType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SettingsChange {
    pub date: DateTime<Utc>,
    pub transaction_type: ServiceTransactionType,
}

#[typetag::serde(name = "Transfer")]
impl ServiceTransaction for Transfer {}

#[typetag::serde(name = "SettingsChange")]
impl ServiceTransaction for SettingsChange {}