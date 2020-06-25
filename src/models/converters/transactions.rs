extern crate chrono;

use super::super::backend::transactions::{ModuleTransaction, MultisigTransaction, EthereumTransaction};
use super::super::service::transactions::Transaction;
use chrono::Utc;
use crate::models::commons::TransactionType;

impl MultisigTransaction {
    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            to: self.to,
            timestamp: self.submission_date.unwrap_or(Utc::now()), // TODO unacceptable default value
            transaction_type: self.tx_type.unwrap_or(TransactionType::MultisigTransaction),
        }
    }
}

impl EthereumTransaction {
    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            to: self.to,
            timestamp: self.execution_date, // TODO unacceptable default value
            transaction_type: self.tx_type.unwrap_or(TransactionType::EthereumTransaction),
        }
    }
}

impl ModuleTransaction {
    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            to: self.to,
            timestamp: self.execution_date.unwrap_or(Utc::now()), // TODO unacceptable default value
            transaction_type: self.tx_type.unwrap_or(TransactionType::ModuleTransaction),
        }
    }
}