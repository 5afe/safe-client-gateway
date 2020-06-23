use super::super::backend::transactions::Transaction as TransactionDto;
use super::super::service::transactions::Transaction;

impl TransactionDto {

    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            to: self.to,
            timestamp: self.submission_date.unwrap(),
        }
    }
}