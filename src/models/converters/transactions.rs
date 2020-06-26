extern crate chrono;

use super::super::backend::transactions::{ModuleTransaction, MultisigTransaction, EthereumTransaction};
use chrono::Utc;
use crate::models::commons::{TransactionType, ServiceTransactionType};
use crate::models::service::transactions::{SettingsChange, Transfer, ServiceTransaction};
use self::chrono::DateTime;

impl MultisigTransaction {
    pub fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.submission_date.unwrap_or(Utc::now()),
            transaction_type: ServiceTransactionType::SettingsChange,
        }
    }

    pub fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
            transaction_type: ServiceTransactionType::Transfer,
        }
    }
}

impl EthereumTransaction {
    pub fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.execution_date,
            transaction_type: ServiceTransactionType::SettingsChange,
        }
    }

    pub fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
            transaction_type: ServiceTransactionType::Transfer,
        }
    }
}

impl ModuleTransaction {
    pub fn to_service_transaction(&self) -> Transfer {
        Transfer {
            to: self.to,
            transaction_type: ServiceTransactionType::Transfer,
        }
    }
}
