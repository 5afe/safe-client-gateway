use crate::models::service::transactions::Transaction::Unknown;
use crate::models::service::transactions::Transaction;
use crate::models::backend::transfers::Transfer;
use crate::models::commons::TransferType;

impl Transfer {
    pub fn to_transfer(&self) -> Transaction {
        match self {
            Transfer::Erc721(transfer) =>
                Transaction::Transfer {
                    to: transfer.to,
                    // block_number: transfer.block_number,
                    // execution_date: transfer.execution_date,
                    transafer_type: TransferType::Erc721,
                },
            Transfer::Erc20(transfer) =>
                Transaction::Transfer {
                    to: transfer.to,
                    transafer_type: TransferType::Erc20,
                },
            Transfer::Ether(transfer) =>
                Transaction::Transfer {
                    to: transfer.to,
                    transafer_type: TransferType::Ether,
                },
            _ => Unknown
        }
    }
}