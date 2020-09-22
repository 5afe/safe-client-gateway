extern crate chrono;

pub mod details;
pub mod summary;

#[cfg(test)]
mod tests;

use super::get_transfer_direction;
use crate::utils::TRANSFER_METHOD;
use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::commons::{DataDecoded, Operation};
use crate::models::service::transactions::{
    Custom, Erc20Transfer, Erc721Transfer, EtherTransfer, SettingsChange, TransactionInfo,
    TransactionStatus, Transfer, TransferDirection, TransferInfo,
};
use crate::providers::info::{InfoProvider, SafeInfo, TokenInfo, TokenType};

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
            TransactionInfo::Custom(self.to_custom())
        } else if value > 0 && data_size == 0 {
            TransactionInfo::Transfer(self.to_ether_transfer())
        } else if value == 0 && data_size > 0 && self.safe == self.to && self.data_decoded.as_ref().map_or_else(|| false, |it| it.is_settings_change()) {
            TransactionInfo::SettingsChange(self.to_settings_change())
        } else if self.data_decoded.as_ref().map(|data_decoded|
            data_decoded.is_erc20_transfer_method() || data_decoded.is_erc721_transfer_method()).unwrap_or(false) &&
            check_sender_or_receiver(&self.data_decoded, &self.safe) {
            info_provider.token_info(&self.to)
                .map_or(TransactionInfo::Custom(self.to_custom()),
                        |token|
                            match token.token_type {
                                TokenType::Erc20 => TransactionInfo::Transfer(self.to_erc20_transfer(&token)),
                                TokenType::Erc721 => TransactionInfo::Transfer(self.to_erc721_transfer(&token)),
                                _ => TransactionInfo::Custom(self.to_custom()),
                            },
                )
        } else {
            TransactionInfo::Custom(self.to_custom())
        }
    }

    fn to_erc20_transfer(&self, token: &TokenInfo) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(&self.data_decoded, "0x0");
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender,
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

    fn to_erc721_transfer(&self, token: &TokenInfo) -> Transfer {
        let sender = get_from_param(&self.data_decoded, &self.safe);
        let recipient = get_to_param(&self.data_decoded, "0x0");
        let direction = get_transfer_direction(&self.safe, &sender, &recipient);
        Transfer {
            sender,
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
                    .and_then(|it|
                        match it.get_parameter_single_value("tokenId") {
                            Some(e) => Some(e),
                            None => it.get_parameter_single_value("value"),
                        })
                    .unwrap_or(String::from("0")),
            }),
        }
    }

    fn to_ether_transfer(&self) -> Transfer {
        Transfer {
            sender: self.safe.to_owned(),
            recipient: self.to.to_owned(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Ether(EtherTransfer {
                value: self.value.as_ref().unwrap().to_string(),
            }),
        }
    }

    fn to_settings_change(&self) -> SettingsChange {
        SettingsChange {
            data_decoded: self.data_decoded.as_ref().unwrap().to_owned(),
            settings_info: self.data_decoded.as_ref().and_then(|it| it.to_settings_info()),
        }
    }

    fn to_custom(&self) -> Custom {
        Custom {
            to: self.to.to_owned(),
            data_size: data_size(&self.data).to_string(),
            value: self.value.as_ref().unwrap().into(),
            method_name: self.data_decoded.as_ref().map(|it| it.method.to_owned()),
        }
    }

    fn value_as_uint(&self) -> u128 {
        self.value.as_ref().map(|it| it.parse::<u128>().ok()).flatten().unwrap_or(0)
    }
}

impl ModuleTransaction {
    fn to_transaction_info(&self) -> TransactionInfo {
        TransactionInfo::Custom(Custom {
            to: self.to.to_owned(),
            data_size: data_size(&self.data).to_string(),
            value: self.value.as_ref().unwrap_or(&String::from("0")).clone(),
            method_name: self.data_decoded.as_ref().map(|it| it.method.to_owned()),
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
    data_decoded.as_ref()
        .and_then(|it| match it.get_parameter_single_value("from") {
            Some(e) => Some(e),
            None => it.get_parameter_single_value("_from"),
        }).unwrap_or(String::from(fallback))
}

fn get_to_param(data_decoded: &Option<DataDecoded>, fallback: &str) -> String {
    data_decoded.as_ref()
        .and_then(|it| match it.get_parameter_single_value("to") {
            Some(e) => Some(e),
            None => it.get_parameter_single_value("_to"),
        })
        .unwrap_or(String::from(fallback))
}

fn check_sender_or_receiver(data_decoded: &Option<DataDecoded>, expected: &String) -> bool {
    if data_decoded.is_none() { return false; };
    let data = data_decoded.as_ref().unwrap();
    data.method == TRANSFER_METHOD
        || &get_from_param(data_decoded, "") == expected
        || &get_to_param(data_decoded, "") == expected
}
