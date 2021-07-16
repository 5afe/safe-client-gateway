extern crate chrono;

use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::converters::transactions::safe_app_info::safe_app_info_from;
use crate::models::service::addresses::AddressEx;
use crate::models::service::transactions::details::{
    DetailedExecutionInfo, ModuleExecutionDetails, MultisigConfirmation, MultisigExecutionDetails,
    TransactionData, TransactionDetails,
};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo, TokenInfo};
use crate::utils::errors::ApiResult;
use rocket::futures::future::OptionFuture;

impl MultisigTransaction {
    pub async fn to_transaction_details(
        &self,
        rejections: Option<Vec<String>>,
        info_provider: &(impl InfoProvider + Sync),
    ) -> ApiResult<TransactionDetails> {
        let safe_info = info_provider
            .safe_info(&self.safe_transaction.safe.to_string())
            .await?;
        let gas_token = info_provider.address_to_token_info(&self.gas_token).await;

        Ok(TransactionDetails {
            executed_at: self.execution_date.map(|data| data.timestamp_millis()),
            tx_status: self.map_status(&safe_info),
            tx_info: self.transaction_info(info_provider).await,
            tx_data: Some(TransactionData {
                to: AddressEx::address_only(&self.safe_transaction.to),
                value: self.safe_transaction.value.to_owned(),
                hex_data: self.safe_transaction.data.to_owned(),
                data_decoded: self.safe_transaction.data_decoded.clone(),
                operation: self.safe_transaction.operation,
                address_info_index: OptionFuture::from(
                    self.safe_transaction
                        .data_decoded
                        .as_ref()
                        .map(|data_decoded| async move {
                            data_decoded.build_address_info_index(info_provider).await
                        }),
                )
                .await
                .flatten(),
            }),
            tx_hash: self.transaction_hash.as_ref().map(|hash| hash.to_owned()),
            detailed_execution_info: Some(DetailedExecutionInfo::Multisig(
                self.build_execution_details(safe_info, gas_token, rejections),
            )),
            safe_app_info: OptionFuture::from(
                self.origin
                    .as_ref()
                    .map(|origin| async move { safe_app_info_from(origin, info_provider).await }),
            )
            .await
            .flatten(),
        })
    }

    fn build_execution_details(
        &self,
        safe_info: SafeInfo,
        gas_token_info: Option<TokenInfo>,
        rejections: Option<Vec<String>>,
    ) -> MultisigExecutionDetails {
        MultisigExecutionDetails {
            submitted_at: self.submission_date.timestamp_millis(),
            nonce: self.nonce,
            safe_tx_hash: self.safe_tx_hash.to_owned(),
            executor: self
                .executor
                .as_ref()
                .map(|address| AddressEx::address_only(&address)),
            signers: safe_info
                .owners
                .iter()
                .map(|rejection| AddressEx::address_only(rejection))
                .collect(),
            confirmations_required: self.confirmations_required.unwrap_or(safe_info.threshold),
            confirmations: self
                .confirmations
                .as_ref()
                .unwrap_or(&vec![])
                .into_iter()
                .map(|confirmation| MultisigConfirmation {
                    signer: AddressEx::address_only(&confirmation.owner),
                    signature: confirmation.signature.to_owned(),
                    submitted_at: confirmation.submission_date.timestamp_millis(),
                })
                .collect(),
            refund_receiver: self
                .refund_receiver
                .as_ref()
                .map(|address| AddressEx::address_only(address))
                .unwrap_or(AddressEx::zero()),
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
            rejectors: rejections.map(|r| {
                r.iter()
                    .map(|rejection| AddressEx::address_only(rejection))
                    .collect()
            }),
        }
    }
}

impl ModuleTransaction {
    pub async fn to_transaction_details(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> ApiResult<TransactionDetails> {
        let safe_transaction = &self.safe_transaction;
        let module_info = info_provider
            .address_ex_from_contracts_or_default(&self.module)
            .await;
        Ok(TransactionDetails {
            executed_at: Some(self.execution_date.timestamp_millis()),
            tx_status: self.map_status(),
            tx_info: self.transaction_info(info_provider).await,
            tx_data: Some(TransactionData {
                to: AddressEx::address_only(&safe_transaction.to),
                value: safe_transaction.value.to_owned(),
                hex_data: safe_transaction.data.to_owned(),
                data_decoded: safe_transaction.data_decoded.clone(),
                operation: safe_transaction.operation,
                address_info_index: OptionFuture::from(safe_transaction.data_decoded.as_ref().map(
                    |data_decoded| async move {
                        data_decoded.build_address_info_index(info_provider).await
                    },
                ))
                .await
                .flatten(),
            }),
            tx_hash: Some(self.transaction_hash.to_owned()),
            detailed_execution_info: Some(DetailedExecutionInfo::Module(ModuleExecutionDetails {
                address: module_info,
            })),
            safe_app_info: None,
        })
    }
}
