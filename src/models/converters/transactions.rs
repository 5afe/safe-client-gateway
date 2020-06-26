extern crate chrono;

use super::super::backend::transactions::{Transaction, ModuleTransaction, MultisigTransaction, EthereumTransaction};
use chrono::Utc;
use crate::models::service::transactions::{SettingsChange, Transfer, ServiceTransaction};

#[typetag::serde(name = "MULTISIG_TRANSACTION")]
impl Transaction for MultisigTransaction {
    fn to_service_transaction(&self) -> Box<dyn ServiceTransaction> {
        Box::new(self.to_settings_change())
    }
}

#[typetag::serde(name = "ETHEREUM_TRANSACTION")]
impl Transaction for EthereumTransaction {
    fn to_service_transaction(&self) -> Box<dyn ServiceTransaction> {
        Box::new(self.to_transfer())
    }
}

#[typetag::serde(name = "MODULE_TRANSACTION")]
impl Transaction for ModuleTransaction {
    fn to_service_transaction(&self) -> Box<dyn ServiceTransaction> {
        Box::new(self.some_mapping_magic())
    }
}

impl MultisigTransaction {
    fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.submission_date.unwrap_or(Utc::now()),
        }
    }

    #[allow(dead_code)]
    fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
        }
    }
}

impl EthereumTransaction {
    #[allow(dead_code)]
    fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.execution_date,
        }
    }

    fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
        }
    }
}

impl ModuleTransaction {
    fn some_mapping_magic(&self) -> SettingsChange {
        SettingsChange {
            date: self.execution_date.unwrap_or(Utc::now()),
        }
    }
}