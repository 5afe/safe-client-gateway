extern crate chrono;

pub mod details;
pub mod safe_app_info;
pub mod summary;

#[cfg(test)]
mod tests;

use super::get_transfer_direction;
use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::commons::{DataDecoded, Operation};
use crate::models::converters::get_address_info;
use crate::models::service::transactions::{
    Custom, Erc20Transfer, Erc721Transfer, EtherTransfer, SettingsChange, TransactionInfo,
    TransactionStatus, Transfer, TransferDirection, TransferInfo,
};
use crate::providers::info::{InfoProvider, SafeInfo, TokenInfo, TokenType};
use crate::utils::TRANSFER_METHOD;

impl MultisigTransaction {
    fn confirmation_count(&self) -> u64 {
        match &self.confirmations {
            Some(confirmations) => confirmations.len() as u64,
            None => 0,
        }
    }

    fn confirmation_required(&self, threshold: u64) -> u64 {
        self.confirmations_required.unwrap_or(threshold)
    }

    fn missing_signers(&self, owners: &Vec<String>) -> Vec<String> {
        self.confirmations.as_ref().map_or_else(
            || owners.to_owned(),
            |confirmations| {
                owners
                    .iter()
                    .filter_map(|owner| {
                        if !confirmations.iter().any(|c| &c.owner == owner) {
                            Some(owner.to_owned())
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

    fn transaction_info(&self, info_provider: &mut dyn InfoProvider) -> TransactionInfo {
        let value = self.value_as_uint();
        let data_size = data_size(&self.data);

        if (value > 0 && data_size > 0) || !self.operation.contains(&Operation::CALL) {
            TransactionInfo::Custom(self.to_custom(info_provider))
        } else if value > 0 && data_size == 0 {
            TransactionInfo::Transfer(self.to_ether_transfer(info_provider))
        } else if value == 0
            && data_size > 0
            && self.safe == self.to
            && self
                .data_decoded
                .as_ref()
                .map_or_else(|| false, |it| it.is_settings_change())
        {
            TransactionInfo::SettingsChange(self.to_settings_change(info_provider))
        } else if self
            .data_decoded
            .as_ref()
            .map(|data_decoded| {
                data_decoded.is_erc20_transfer_method() || data_decoded.is_erc721_transfer_method()
            })
            .unwrap_or(false)
            && check_sender_or_receiver(&self.data_decoded, &self.safe)
        {
            match info_provider.token_info(&self.to) {
                Ok(token) => match token.token_type {
                    TokenType::Erc20 => {
                        TransactionInfo::Transfer(self.to_erc20_transfer(&token, info_provider))
                    }
                    TokenType::Erc721 => {
                        TransactionInfo::Transfer(self.to_erc721_transfer(&token, info_provider))
                    }
                    _ => TransactionInfo::Custom(self.to_custom(info_provider)),
                },
                _ => TransactionInfo::Custom(self.to_custom(info_provider)),
            }
        } else {
            TransactionInfo::Custom(self.to_custom(info_provider))
        }
    }

    fn to_erc20_transfer(
        &self,
        token: &TokenInfo,
        info_provider: &mut dyn InfoProvider,
    ) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(&self.data_decoded, "0x0");
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender_info: get_address_info(&self.safe, &sender, info_provider),
            sender,
            recipient_info: get_address_info(&self.safe, &recipient, info_provider),
            recipient,
            direction,
            transfer_info: TransferInfo::Erc20(Erc20Transfer {
                token_address: token.address.to_owned(),
                logo_uri: token.logo_uri.to_owned(),
                token_name: Some(token.name.to_owned()),
                token_symbol: Some(token.symbol.to_owned()),
                decimals: Some(token.decimals),
                value: self
                    .data_decoded
                    .as_ref()
                    .and_then(|it| it.get_parameter_single_value("value"))
                    .unwrap_or(String::from("0")),
            }),
        }
    }

    fn to_erc721_transfer(
        &self,
        token: &TokenInfo,
        info_provider: &mut dyn InfoProvider,
    ) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(&self.data_decoded, "0x0");
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender_info: get_address_info(&self.safe, &sender, info_provider),
            sender,
            recipient_info: get_address_info(&self.safe, &recipient, info_provider),
            recipient,
            direction,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: token.address.to_owned(),
                logo_uri: token.logo_uri.to_owned(),
                token_name: Some(token.name.to_owned()),
                token_symbol: Some(token.symbol.to_owned()),
                token_id: self
                    .data_decoded
                    .as_ref()
                    .and_then(|it| match it.get_parameter_single_value("tokenId") {
                        Some(e) => Some(e),
                        None => it.get_parameter_single_value("value"),
                    })
                    .unwrap_or(String::from("0")),
            }),
        }
    }

    fn to_ether_transfer(&self, info_provider: &mut dyn InfoProvider) -> Transfer {
        Transfer {
            sender_info: None,
            sender: self.safe.to_owned(),
            recipient_info: info_provider.full_address_info_search(&self.to).ok(),
            recipient: self.to.to_owned(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Ether(EtherTransfer {
                value: self.value.as_ref().unwrap().to_string(),
            }),
        }
    }

    fn to_settings_change(&self, info_provider: &mut dyn InfoProvider) -> SettingsChange {
        SettingsChange {
            data_decoded: self.data_decoded.as_ref().unwrap().to_owned(),
            settings_info: self
                .data_decoded
                .as_ref()
                .and_then(|it| it.to_settings_info(info_provider)),
        }
    }

    fn to_custom(&self, info_provider: &mut dyn InfoProvider) -> Custom {
        Custom {
            to: self.to.to_owned(),
            to_info: info_provider.full_address_info_search(&self.to).ok(),
            is_cancellation: self.is_cancellation(),
            data_size: data_size(&self.data).to_string(),
            value: self.value.as_ref().unwrap().into(),
            method_name: self.data_decoded.as_ref().map(|it| it.method.to_owned()),
            action_count: self
                .data_decoded
                .as_ref()
                .and_then(|it| it.get_action_count()),
        }
    }

    fn value_as_uint(&self) -> u128 {
        self.value
            .as_ref()
            .map(|it| it.parse::<u128>().ok())
            .flatten()
            .unwrap_or(0)
    }

    fn is_cancellation(&self) -> bool {
        self.to == self.safe
            && data_size(&self.data) == 0
            && self.value.as_ref().map_or(true, |value| value == "0")
            && self
                .operation
                .map_or(true, |operation| operation == Operation::CALL)
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
    fn to_transaction_info(&self, info_provider: &mut dyn InfoProvider) -> TransactionInfo {
        TransactionInfo::Custom(Custom {
            to_info: info_provider.full_address_info_search(&self.to).ok(),
            to: self.to.to_owned(),
            data_size: data_size(&self.data).to_string(),
            value: self.value.as_ref().unwrap_or(&String::from("0")).clone(),
            method_name: self.data_decoded.as_ref().map(|it| it.method.to_owned()),
            action_count: self
                .data_decoded
                .as_ref()
                .and_then(|it| it.get_action_count()),
            is_cancellation: false,
        })
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
        .and_then(|it| match it.get_parameter_single_value("from") {
            Some(e) => Some(e),
            None => it.get_parameter_single_value("_from"),
        })
        .unwrap_or(String::from(fallback))
}

fn get_to_param(data_decoded: &Option<DataDecoded>, fallback: &str) -> String {
    data_decoded
        .as_ref()
        .and_then(|it| match it.get_parameter_single_value("to") {
            Some(e) => Some(e),
            None => it.get_parameter_single_value("_to"),
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
