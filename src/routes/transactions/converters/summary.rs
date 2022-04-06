use crate::common::models::backend::transactions::{
    CreationTransaction, EthereumTransaction, ModuleTransaction, MultisigTransaction, Transaction,
};
use crate::common::models::backend::transfers::Transfer;
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::InfoProvider;
use crate::routes::transactions::converters::safe_app_info::safe_app_info_from;
use crate::routes::transactions::models::summary::{
    ExecutionInfo, ModuleExecutionInfo, MultisigExecutionInfo, TransactionSummary,
};
use crate::routes::transactions::models::{Creation, TransactionInfo, TransactionStatus};
use crate::utils::errors::ApiResult;
use rocket::futures::future::OptionFuture;

impl Transaction {
    pub async fn to_transaction_summary(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        safe: &str,
    ) -> ApiResult<Vec<TransactionSummary>> {
        match self {
            Transaction::Multisig(transaction) => {
                Ok(transaction.to_transaction_summary(info_provider).await?)
            }
            Transaction::Ethereum(transaction) => Ok(transaction
                .to_transaction_summary(info_provider, safe)
                .await),
            Transaction::Module(transaction) => {
                Ok(transaction.to_transaction_summary(info_provider).await)
            }
            Transaction::Unknown => bail!("Unknown transaction type from backend"),
        }
    }
}

impl MultisigTransaction {
    pub async fn to_transaction_summary(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> ApiResult<Vec<TransactionSummary>> {
        let safe_info = info_provider
            .safe_info(&self.safe_transaction.safe.to_string())
            .await?;
        let tx_status = self.map_status(&safe_info);
        let missing_signers = if tx_status == TransactionStatus::AwaitingConfirmations {
            Some(self.missing_signers(&safe_info.owners))
        } else {
            None
        };
        Ok(vec![TransactionSummary {
            id: self.generate_id(),
            timestamp: self
                .execution_date
                .unwrap_or(self.submission_date)
                .timestamp_millis(),
            tx_status,
            execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
                nonce: self.nonce,
                confirmations_submitted: self.confirmation_count(),
                confirmations_required: self.confirmation_required(safe_info.threshold),
                missing_signers,
            })),
            tx_info: self.transaction_info(info_provider).await,
            safe_app_info: OptionFuture::from(
                self.origin
                    .as_ref()
                    .map(|origin| async move { safe_app_info_from(&origin, info_provider).await }),
            )
            .await
            .flatten(),
        }])
    }
}

impl EthereumTransaction {
    pub(super) async fn to_transaction_summary(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        safe_address: &str,
    ) -> Vec<TransactionSummary> {
        match &self.transfers {
            Some(transfers) => {
                let mut results = Vec::with_capacity(transfers.len());

                for transfer in transfers {
                    let transaction_summary = transfer
                        .to_transaction_summary(
                            info_provider,
                            self.execution_date.timestamp_millis(),
                            safe_address,
                        )
                        .await;
                    results.push(transaction_summary);
                }
                results
            }
            _ => vec![],
        }
    }
}

impl Transfer {
    pub async fn to_transaction_summary(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        execution_date: i64,
        safe_address: &str,
    ) -> TransactionSummary {
        TransactionSummary {
            id: self.generate_id(
                safe_address,
                &self
                    .get_transaction_hash()
                    .expect("No tx hash for ethereum transaction is not possible"),
            ), //TODO is this correct?
            timestamp: execution_date,
            tx_status: TransactionStatus::Success,
            execution_info: None,
            safe_app_info: None,
            tx_info: self.to_transfer(info_provider, safe_address).await,
        }
    }
}

impl ModuleTransaction {
    pub(super) async fn to_transaction_summary(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> Vec<TransactionSummary> {
        let module_info = info_provider
            .address_ex_from_contracts_or_default(&self.module)
            .await;
        vec![TransactionSummary {
            id: self.generate_id(),
            timestamp: self.execution_date.timestamp_millis(),
            tx_status: self.map_status(),
            execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
                address: module_info,
            })),
            safe_app_info: None,
            tx_info: self.transaction_info(info_provider).await,
        }]
    }
}

impl CreationTransaction {
    pub async fn to_transaction_summary(
        &self,
        safe_address: &str,
        info_provider: &(impl InfoProvider + Sync),
    ) -> TransactionSummary {
        TransactionSummary {
            id: self.generate_id(safe_address),
            timestamp: self.created.timestamp_millis(),
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Creation(Creation {
                creator: info_provider
                    .address_ex_from_contracts_or_default(&self.creator)
                    .await,
                transaction_hash: self.transaction_hash.clone(),
                implementation: info_provider
                    .optional_address_ex_from_contracts(&self.master_copy)
                    .await,
                factory: info_provider
                    .optional_address_ex_from_contracts(&self.factory_address)
                    .await,
            }),
            execution_info: None,
            safe_app_info: None,
        }
    }
}
