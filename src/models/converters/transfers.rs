use crate::models::service::transactions::Transaction::Unknown;
use crate::models::service::transactions::Transaction;
use crate::models::backend::transfers::Transfer;

impl Transfer {

    pub fn to_transfer(&self) -> Transaction {
        match self {
            Transfer::Erc721(transfer) =>
                Transaction::Transfer {
                    to: transfer.to,
                    // block_number: transfer.block_number,
                    // execution_date: transfer.execution_date,
                },
            _ => Unknown
        }
    }
}