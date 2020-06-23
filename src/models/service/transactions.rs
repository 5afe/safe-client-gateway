use ethereum_types::{Address, H256, U256};
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub struct Transaction {
    pub to: Address,
    pub timestamp: DateTime<Utc>,
}