extern crate chrono;

use super::super::backend::transactions::{ModuleTransaction, MultisigTransaction, EthereumTransaction};
use chrono::Utc;
use crate::models::service::transactions::{SettingsChange, Transfer};

impl MultisigTransaction {
    pub fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.submission_date.unwrap_or(Utc::now()),
        }
    }

    pub fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
        }
    }
}

impl EthereumTransaction {
    pub fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            date: self.execution_date,
        }
    }

    pub fn to_transfer(&self) -> Transfer {
        Transfer {
            to: self.to,
        }
    }
}

impl ModuleTransaction {
    pub fn to_service_transaction(&self) -> Transfer {
        Transfer {
            to: self.to,
        }
    }
}
