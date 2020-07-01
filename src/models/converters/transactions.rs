extern crate chrono;

use super::super::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::{Transaction as ServiceTransaction, SettingsChange, Custom as CustomTransaction};
use crate::models::backend::transactions::{MultisigTransaction, ModuleTransaction, EthereumTransaction};
use crate::models::commons::Operation;

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
    fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        vec!(match &self.data_decoded {
            Some(data_decoded) =>
                ServiceTransaction::SettingsChange(
                    SettingsChange {
                        data_decoded: data_decoded.clone(),
                    }
                ),
            None =>
                ServiceTransaction::Custom(
                    CustomTransaction {
                        to: self.to
                    }
                )
        })
    }

    #[allow(dead_code)]
    fn is_erc20_transfer(&self) -> bool {
        self.operation.filter(|&operation| operation == Operation::CALL).is_some()
    }

    // fn isErc721Transfer(&self) -> bool{}
    // fn isEtherTransfer(&self) -> bool{}
    // fn isSettingsChange(&self) -> bool{}
}

impl EthereumTransaction {
    fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        match &self.transfers {
            Some(transfers) => transfers.into_iter()
                .map(|transfer| transfer.to_transfer())
                .collect(),
            _ => vec!()
        }
    }
}

impl ModuleTransaction {
    fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
        vec!(ServiceTransaction::Custom(
            CustomTransaction {
                to: self.to
            })
        )
    }
}
