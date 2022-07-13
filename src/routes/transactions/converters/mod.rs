pub mod details;
pub mod safe_app_info;
pub mod summary;
pub mod transaction_id;

#[cfg(test)]
mod tests;

use crate::common::converters::{get_address_ex_from_any_source, get_transfer_direction};
use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::transactions::{
    ModuleTransaction, MultisigTransaction, SafeTransaction,
};
use crate::common::models::data_decoded::{DataDecoded, Operation};
use crate::providers::info::{InfoProvider, SafeInfo, TokenInfo, TokenType};
use crate::routes::transactions::models::{
    Custom, Erc20Transfer, Erc721Transfer, NativeCoinTransfer, SettingsChange, TransactionInfo,
    TransactionStatus, Transfer, TransferDirection, TransferInfo,
};
use crate::utils::{SAFE_TRANSFER_FROM_METHOD, TRANSFER_FROM_METHOD, TRANSFER_METHOD};
use rocket::futures::future::OptionFuture;

impl SafeTransaction {
    pub(crate) async fn transaction_info(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        is_cancellation: bool,
    ) -> TransactionInfo {
        let value = self.value_as_uint();
        let data_size = data_size(&self.data);

        if (value > 0 && data_size > 0) || self.operation != Operation::CALL {
            TransactionInfo::Custom(self.to_custom(info_provider, is_cancellation).await)
        } else if value > 0 && data_size == 0 {
            TransactionInfo::Transfer(self.to_ether_transfer(info_provider).await)
        } else if value == 0
            && data_size > 0
            && self.safe == self.to
            && self
                .data_decoded
                .as_ref()
                .map_or_else(|| false, |it| it.is_settings_change())
        {
            TransactionInfo::SettingsChange(self.to_settings_change(info_provider).await)
        } else if self
            .data_decoded
            .as_ref()
            .map(|data_decoded| {
                data_decoded.is_erc20_transfer_method() || data_decoded.is_erc721_transfer_method()
            })
            .unwrap_or(false)
            && check_sender_or_receiver(&self.data_decoded, &self.safe)
        {
            match info_provider.token_info(&self.to).await {
                Ok(token) => match token.token_type {
                    TokenType::Erc20 => TransactionInfo::Transfer(
                        self.to_erc20_transfer(&token, info_provider).await,
                    ),
                    TokenType::Erc721 => TransactionInfo::Transfer(
                        self.to_erc721_transfer(&token, info_provider).await,
                    ),
                    _ => TransactionInfo::Custom(
                        self.to_custom(info_provider, is_cancellation).await,
                    ),
                },
                _ => TransactionInfo::Custom(self.to_custom(info_provider, is_cancellation).await),
            }
        } else {
            TransactionInfo::Custom(self.to_custom(info_provider, is_cancellation).await)
        }
    }

    async fn to_custom(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        is_cancellation: bool,
    ) -> Custom {
        Custom {
            to: crate::common::converters::get_address_ex_from_any_source(
                &self.safe,
                &self.to,
                info_provider,
            )
            .await,
            is_cancellation,
            data_size: data_size(&self.data).to_string(),
            value: self.value.as_ref().unwrap_or(&String::from("0")).clone(),
            method_name: self.data_decoded.as_ref().map(|it| it.method.to_owned()),
            action_count: self
                .data_decoded
                .as_ref()
                .and_then(|it| it.get_action_count()),
        }
    }

    async fn to_erc20_transfer(
        &self,
        token: &TokenInfo,
        info_provider: &impl InfoProvider,
    ) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(
            &self.data_decoded,
            "0x0000000000000000000000000000000000000000",
        );
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender: get_address_ex_from_any_source(&self.safe, &sender, info_provider).await,
            recipient: get_address_ex_from_any_source(&self.safe, &recipient, info_provider).await,
            direction,
            transfer_info: TransferInfo::Erc20(Erc20Transfer {
                token_address: token.address.to_owned(),
                logo_uri: token.logo_uri.to_owned(),
                token_name: Some(token.name.to_owned()),
                token_symbol: Some(token.symbol.to_owned()),
                decimals: Some(token.decimals),
                value: get_value_param(&self.data_decoded, "0"),
            }),
        }
    }

    async fn to_erc721_transfer(
        &self,
        token: &TokenInfo,
        info_provider: &impl InfoProvider,
    ) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(
            &self.data_decoded,
            "0x0000000000000000000000000000000000000000",
        );
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender: get_address_ex_from_any_source(&self.safe, &sender, info_provider).await,
            recipient: get_address_ex_from_any_source(&self.safe, &recipient, info_provider).await,
            direction,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: token.address.to_owned(),
                logo_uri: token.logo_uri.to_owned(),
                token_name: Some(token.name.to_owned()),
                token_symbol: Some(token.symbol.to_owned()),
                token_id: get_value_param(&self.data_decoded, "0"),
            }),
        }
    }

    async fn to_ether_transfer(&self, info_provider: &impl InfoProvider) -> Transfer {
        Transfer {
            sender: AddressEx::address_only(&self.safe),
            recipient: get_address_ex_from_any_source(&self.safe, &self.to, info_provider).await,
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
                value: self.value.as_ref().unwrap().to_string(),
            }),
        }
    }

    async fn to_settings_change(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> SettingsChange {
        SettingsChange {
            data_decoded: self.data_decoded.as_ref().unwrap().to_owned(),
            settings_info: OptionFuture::from(
                self.data_decoded
                    .as_ref()
                    .map(|it| async move { it.to_settings_info(info_provider).await }),
            )
            .await
            .flatten(),
        }
    }

    fn value_as_uint(&self) -> u128 {
        self.value
            .as_ref()
            .map(|it| it.parse::<u128>().ok())
            .flatten()
            .unwrap_or(0)
    }
}

impl MultisigTransaction {
    async fn transaction_info(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> TransactionInfo {
        self.safe_transaction
            .transaction_info(info_provider, self.is_cancellation())
            .await
    }

    fn confirmation_count(&self) -> u64 {
        match &self.confirmations {
            Some(confirmations) => confirmations.len() as u64,
            None => 0,
        }
    }

    fn confirmation_required(&self, threshold: u64) -> u64 {
        self.confirmations_required.unwrap_or(threshold)
    }

    fn missing_signers(&self, owners: &Vec<String>) -> Vec<AddressEx> {
        self.confirmations.as_ref().map_or_else(
            || {
                owners
                    .iter()
                    .map(|owner| AddressEx::address_only(&owner))
                    .collect()
            },
            |confirmations| {
                owners
                    .iter()
                    .filter_map(|owner| {
                        if !confirmations.iter().any(|c| &c.owner == owner) {
                            Some(AddressEx::address_only(&owner))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
        )
    }

    fn map_status(&self, safe_info: &SafeInfo) -> TransactionStatus {
        if self.is_executed {
            if self.is_successful.unwrap_or(false) {
                TransactionStatus::Success
            } else {
                TransactionStatus::Failed
            }
        } else if safe_info.nonce > self.nonce {
            TransactionStatus::Cancelled
        } else if self.confirmation_count() < self.confirmation_required(safe_info.threshold) {
            TransactionStatus::AwaitingConfirmations
        } else {
            TransactionStatus::AwaitingExecution
        }
    }

    fn is_cancellation(&self) -> bool {
        self.safe_transaction.to == self.safe_transaction.safe
            && data_size(&self.safe_transaction.data) == 0
            && self
                .safe_transaction
                .value
                .as_ref()
                .map_or(true, |value| value == "0")
            && self.safe_transaction.operation == Operation::CALL
            && self
                .base_gas
                .as_ref()
                .map_or(true, |base_gas| base_gas.eq(&0))
            && self
                .gas_price
                .as_ref()
                .map_or(true, |gas_price| gas_price == "0")
            && self.gas_token.as_ref().map_or(true, |gas_token| {
                gas_token == "0x0000000000000000000000000000000000000000"
            })
            && self
                .refund_receiver
                .as_ref()
                .map_or(true, |refund_receiver| {
                    refund_receiver == "0x0000000000000000000000000000000000000000"
                })
            && self
                .safe_tx_gas
                .as_ref()
                .map_or(true, |safe_tx_gas| safe_tx_gas.eq(&0))
    }
}

impl ModuleTransaction {
    async fn transaction_info(
        &self,
        info_provider: &(impl InfoProvider + Sync),
    ) -> TransactionInfo {
        self.safe_transaction
            .transaction_info(info_provider, false)
            .await
    }

    fn map_status(&self) -> TransactionStatus {
        if self.is_successful {
            TransactionStatus::Success
        } else {
            TransactionStatus::Failed
        }
    }
}

fn data_size(data: &Option<String>) -> usize {
    match data {
        Some(actual_data) => {
            let length = actual_data.len();
            match length {
                0 => 0,
                _ => (length - 2) / 2,
            }
        }
        None => 0,
    }
}

fn get_from_param(data_decoded: &Option<DataDecoded>, fallback: &str) -> String {
    data_decoded
        .as_ref()
        .and_then(|data_decoded| match data_decoded.method.as_str() {
            TRANSFER_METHOD => None,
            TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(0),
            SAFE_TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(0),
            _ => None,
        })
        .unwrap_or(String::from(fallback))
}

fn get_to_param(data_decoded: &Option<DataDecoded>, fallback: &str) -> String {
    data_decoded
        .as_ref()
        .and_then(|data_decoded| match data_decoded.method.as_str() {
            TRANSFER_METHOD => data_decoded.get_parameter_single_value_at(0),
            TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(1),
            SAFE_TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(1),
            _ => None,
        })
        .unwrap_or(String::from(fallback))
}

fn get_value_param(data_decoded: &Option<DataDecoded>, fallback: &str) -> String {
    data_decoded
        .as_ref()
        .and_then(|data_decoded| match data_decoded.method.as_str() {
            TRANSFER_METHOD => data_decoded.get_parameter_single_value_at(1),
            TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(2),
            SAFE_TRANSFER_FROM_METHOD => data_decoded.get_parameter_single_value_at(2),
            _ => None,
        })
        .unwrap_or(String::from(fallback))
}

fn check_sender_or_receiver(data_decoded: &Option<DataDecoded>, expected: &str) -> bool {
    if data_decoded.is_none() {
        return false;
    };
    let data = data_decoded.as_ref().unwrap();
    data.method == TRANSFER_METHOD
        || &get_from_param(data_decoded, "") == expected
        || &get_to_param(data_decoded, "") == expected
}
