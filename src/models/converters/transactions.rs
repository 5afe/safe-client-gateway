extern crate chrono;

use super::super::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::{Transaction as ServiceTransaction, SettingsChange, Custom as CustomTransaction, Transfer, Custom, TransferInfo};
use crate::models::backend::transactions::{MultisigTransaction, ModuleTransaction, EthereumTransaction};
use crate::models::commons::Operation;
use ethereum_types::{Address, H160, H256};

impl TransactionDto {
    pub fn to_service_transaction(&self) -> Vec<ServiceTransaction> {
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
        vec!(
            if self.is_erc20_transfer() {
                ServiceTransaction::Transfer(self.to_erc20_transfer())
            } else if self.is_erc721_transfer() {
                ServiceTransaction::Transfer(self.to_erc721_transfer())
            } else if self.is_ether_transfer() {
                ServiceTransaction::Transfer(self.to_ether_transfer())
            } else if self.is_settings_change() {
                ServiceTransaction::SettingsChange(self.to_settings_change())
            } else {
                ServiceTransaction::Custom(self.to_custom())
            }
        )
    }

    fn is_erc20_transfer(&self) -> bool {
        self.operation.contains(&Operation::CALL)
            && self.data_decoded.is_some()
            && self.data_decoded.as_ref().unwrap().is_erc20_transfer_method()
            && self.data_decoded.as_ref().unwrap().contains_parameter("value")
    }

    fn is_erc721_transfer(&self) -> bool {
        self.operation.contains(&Operation::CALL)
            && self.data_decoded.is_some()
            && self.data_decoded.as_ref().unwrap().is_erc721_transfer_method()
            && self.data_decoded.as_ref().unwrap().contains_parameter("tokenId")
    }

    fn is_ether_transfer(&self) -> bool {
        self.operation.contains(&Operation::CALL)
            && self.data.is_some()
    }

    fn is_settings_change(&self) -> bool {
        self.to.unwrap_or(Address::from(H160::zero())) == self.safe
            && self.operation.contains(&Operation::CALL)
            && self.data_decoded.is_some()
            && self.data_decoded.as_ref().unwrap().is_settings_change()
    }

    fn to_erc20_transfer(&self) -> Transfer {
        Transfer {
            sender: self.safe,
            recipient: self.safe,
            date: self.submission_date,
            transaction_hash: self.transaction_hash.unwrap_or(H256::zero()),
            transfer_info: TransferInfo::Erc20 {
                token_name: String::from("Blabla"),
                token_symbol: String::from("BLA"),
                logo_uri: String::from("some.url"),
                decimals: 12,
                value: self.data_decoded.as_ref().and_then(
                    |it| it.get_parameter_value("value")
                ).unwrap_or(String::from("0")),
            },
        }
    }

    fn to_erc721_transfer(&self) -> Transfer {
        Transfer {
            sender: self.safe,
            recipient: self.safe,
            date: self.submission_date,
            transaction_hash: self.transaction_hash.unwrap_or(H256::zero()),
            transfer_info: TransferInfo::Erc721 {
                token_id:  self.data_decoded.as_ref().and_then(
                    |it| it.get_parameter_value("tokenId")
                ).unwrap_or(String::from("0")),
                token_address: Address::from(H160::zero()),
            },
        }
    }

    fn to_ether_transfer(&self) -> Transfer {
        Transfer {
            sender: self.safe,
            recipient: self.safe,
            date: self.submission_date,
            transaction_hash: self.transaction_hash.unwrap_or(H256::zero()),
            transfer_info: TransferInfo::Ether {
                value: self.value.as_ref().unwrap().to_string(),
            },
        }
    }

    fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            data_decoded: self.data_decoded.as_ref().unwrap().to_owned()
        }
    }

    fn to_custom(&self) -> Custom {
        Custom { to: self.safe }
    }
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
