extern crate chrono;

use super::super::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::Transaction as ServiceTransaction;
use crate::models::backend::transactions::{MultisigTransaction, ModuleTransaction, EthereumTransaction};

pub trait Transaction {
    fn to_service_transaction(&self) -> Vec<ServiceTransaction>;
}

impl Transaction for TransactionDto {
    fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        match self {
            TransactionDto::Multisig(transaction) => transaction.to_service_transaction(),
            TransactionDto::Ethereum(transaction) => transaction.to_service_transaction(),
            TransactionDto::Module(transaction) => transaction.to_service_transaction(),
            TransactionDto::Unknown => {
                println!("Unknown transaction type");
                vec!(ServiceTransaction::Unknown)
            }
        }
    }
}

impl MultisigTransaction {
    pub fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        vec!(ServiceTransaction::Transfer {
            to: self.to
        })
    }
}

impl EthereumTransaction {
    pub fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        match &self.transfers {
            Some(transfers) => transfers.into_iter()
                .map(|transfer| transfer.to_transfer())
                .collect(),
            _ => vec!()
        }
    }
}

impl ModuleTransaction {
    pub fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        vec!(ServiceTransaction::Custom {
            to: self.to
        })
    }
}
