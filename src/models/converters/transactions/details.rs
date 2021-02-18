extern crate chrono;

use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::commons::Operation;
use crate::models::converters::transactions::safe_app_info::safe_app_info_from;
use crate::models::service::transactions::details::{
    DetailedExecutionInfo, ModuleExecutionDetails, MultisigConfirmation, MultisigExecutionDetails,
    TransactionData, TransactionDetails,
};
use crate::models::service::transactions::TransactionStatus;
use crate::providers::info::{InfoProvider, SafeInfo, TokenInfo};
use crate::utils::errors::ApiResult;

impl MultisigTransaction {
    pub fn to_transaction_details(
        &self,
        conflicting_txs: Vec<MultisigTransaction>,
        info_provider: &mut dyn InfoProvider,
    ) -> ApiResult<TransactionDetails> {
        let safe_info = info_provider.safe_info(&self.safe.to_string())?;
        let gas_token = self
            .gas_token
            .as_ref()
            .map(|it| info_provider.token_info(it).ok())
            .flatten();

        Ok(TransactionDetails {
            executed_at: self.execution_date.map(|data| data.timestamp_millis()),
            tx_status: self.map_status(&safe_info),
            tx_info: self.transaction_info(info_provider),
            tx_data: Some(TransactionData {
                to: self.to.to_owned(),
                value: self.value.to_owned(),
                hex_data: self.data.to_owned(),
                data_decoded: self.data_decoded.clone(),
                operation: self.operation.unwrap_or(Operation::CALL),
            }),
            tx_hash: self.transaction_hash.as_ref().map(|hash| hash.to_owned()),
            detailed_execution_info: Some(DetailedExecutionInfo::Multisig(
                self.build_execution_details(safe_info, gas_token, conflicting_txs),
            )),
            safe_app_info: self
                .origin
                .as_ref()
                .and_then(|origin| safe_app_info_from(origin, info_provider)),
        })
    }

    fn build_execution_details(
        &self,
        safe_info: SafeInfo,
        gas_token_info: Option<TokenInfo>,
        conflicting_txs: Vec<MultisigTransaction>,
    ) -> MultisigExecutionDetails {
        MultisigExecutionDetails {
            submitted_at: self.submission_date.timestamp_millis(),
            nonce: self.nonce,
            safe_tx_hash: self.safe_tx_hash.to_owned(),
            executor: self.executor.to_owned(),
            signers: safe_info.owners,
            confirmations_required: self.confirmations_required.unwrap_or(safe_info.threshold),
            confirmations: self
                .confirmations
                .as_ref()
                .unwrap_or(&vec![])
                .into_iter()
                .map(|confirmation| MultisigConfirmation {
                    signer: confirmation.owner.to_owned(),
                    signature: confirmation.signature.to_owned(),
                    submitted_at: confirmation.submission_date.timestamp_millis(),
                })
                .collect(),
            refund_receiver: self
                .refund_receiver
                .as_ref()
                .unwrap_or(&String::from("0x0000000000000000000000000000000000000000"))
                .to_owned(),
            gas_token: self
                .gas_token
                .as_ref()
                .unwrap_or(&String::from("0x0000000000000000000000000000000000000000"))
                .to_owned(),
            base_gas: self.base_gas.unwrap_or(0),
            safe_tx_gas: self.safe_tx_gas.unwrap_or(0),
            gas_price: self
                .gas_price
                .as_ref()
                .unwrap_or(&String::from("0"))
                .to_owned(),
            gas_token_info,
            rejections: self.get_rejections(conflicting_txs),
        }
    }

    fn get_rejections(
        &self,
        conflicting_txs: Vec<MultisigTransaction>,
    ) -> Option<Vec<MultisigConfirmation>> {
        if self.is_cancellation() {
            None
        } else {
            conflicting_txs
                .iter()
                .find(|tx| tx.is_cancellation())
                .map(|tx| {
                    tx.confirmations.as_ref().map(|it| {
                        it.iter()
                            .map(|confirmation| MultisigConfirmation {
                                signer: confirmation.owner.to_owned(),
                                signature: confirmation.signature.to_owned(),
                                submitted_at: confirmation.submission_date.timestamp_millis(),
                            })
                            .collect()
                    })
                })
                .flatten()
        }
    }
}

impl ModuleTransaction {
    pub fn to_transaction_details(
        &self,
        info_provider: &mut dyn InfoProvider,
    ) -> ApiResult<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: Some(self.execution_date.timestamp_millis()),
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transaction_info(info_provider),
            tx_data: Some(TransactionData {
                to: self.to.to_owned(),
                value: self.value.to_owned(),
                hex_data: self.data.to_owned(),
                data_decoded: self.data_decoded.clone(),
                operation: self.operation,
            }),
            tx_hash: Some(self.transaction_hash.to_owned()),
            detailed_execution_info: Some(DetailedExecutionInfo::Module(ModuleExecutionDetails {
                address: self.module.to_owned(),
            })),
            safe_app_info: None,
        })
    }
}
