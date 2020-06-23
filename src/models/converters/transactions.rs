extern crate chrono;
use super::super::backend::transactions::Transaction as TransactionDto;
use super::super::service::transactions::Transaction;
use chrono::Utc;

impl TransactionDto {
    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            to: self.to,
            timestamp: self.submission_date.unwrap_or(Utc::now()), // TODO unacceptable default value
            transaction_type: self.tx_type,
        }
    }
}