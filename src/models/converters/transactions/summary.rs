extern crate chrono;

use crate::models::backend::transactions::Transaction;
use crate::models::backend::transactions::{
    EthereumTransaction, ModuleTransaction, MultisigTransaction,
};
use crate::models::service::transactions::summary::{ExecutionInfo, TransactionSummary};
use crate::models::service::transactions::{
    TransactionStatus, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX,
    ID_SEPERATOR,
};
use crate::providers::info::InfoProvider;
use crate::utils::hex_hash;
use anyhow::{Error, Result};

macro_rules! concat_parts {
    ($parts_head:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}",
            ID_SEPERATOR,
            $parts_head
        );
    };
    ($parts_head:expr, $($parts_tail:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}{}",
            ID_SEPERATOR,
            $parts_head,
            concat_parts!($($parts_tail),+)
        );
    };
}

macro_rules! create_id {
    ($tx_type:expr, $($parts:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!("{}{}", $tx_type, concat_parts!($($parts),+));
    };
}

impl Transaction {
    pub fn to_transaction_summary(
        &self,
        info_provider: &mut InfoProvider,
        safe: &String,
    ) -> Result<Vec<TransactionSummary>> {
        match self {
            Transaction::Multisig(transaction) => {
                Ok(transaction.to_transaction_summary(info_provider)?)
            }
            Transaction::Ethereum(transaction) => {
                Ok(transaction.to_transaction_summary(info_provider, safe))
            }
            Transaction::Module(transaction) => Ok(transaction.to_transaction_summary()),
            Transaction::Unknown => Err(Error::msg("Unknown transaction type from backend")),
        }
    }
}

impl MultisigTransaction {
    fn to_transaction_summary(
        &self,
        info_provider: &mut InfoProvider,
    ) -> Result<Vec<TransactionSummary>> {
        let safe_info = info_provider.safe_info(&self.safe.to_string())?;
        Ok(vec![TransactionSummary {
            id: create_id!(
                ID_PREFIX_MULTISIG_TX, self.safe_tx_hash
            ),
            timestamp: self
                .execution_date
                .unwrap_or(self.submission_date)
                .timestamp_millis(),
            tx_status: self.map_status(&safe_info),
            execution_info: Some(ExecutionInfo {
                nonce: self.nonce,
                confirmations_submitted: self.confirmation_count(),
                confirmations_required: self.confirmation_required(&safe_info),
            }),
            tx_info: self.transaction_info(info_provider),
        }])
    }
}

impl EthereumTransaction {
    fn to_transaction_summary(
        &self,
        info_provider: &mut InfoProvider,
        safe: &String,
    ) -> Vec<TransactionSummary> {
        match &self.transfers {
            Some(transfers) => transfers
                .into_iter()
                .map(|transfer| TransactionSummary {
                    id: create_id!(
                        ID_PREFIX_ETHEREUM_TX,
                        safe,
                        self.block_number,
                        hex_hash(transfer)
                    ),
                    timestamp: self.execution_date.timestamp_millis(),
                    tx_status: TransactionStatus::Success,
                    execution_info: None,
                    tx_info: transfer.to_transfer(info_provider),
                })
                .collect(),
            _ => vec![],
        }
    }
}

impl ModuleTransaction {
    fn to_transaction_summary(&self) -> Vec<TransactionSummary> {
        vec![TransactionSummary {
            id: create_id!(
                ID_PREFIX_MODULE_TX,
                self.safe,
                self.block_number,
                hex_hash(self)
            ),
            timestamp: self.execution_date.timestamp_millis(),
            tx_status: TransactionStatus::Success,
            execution_info: None,
            tx_info: self.to_transaction_info(),
        }]
    }
}
