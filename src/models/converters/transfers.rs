use super::get_transfer_direction;
use crate::models::backend::transfers::{
    Erc20TokenInfo, Erc20Transfer as Erc20TransferDto, Erc721TokenInfo,
    Erc721Transfer as Erc721TransferDto, EtherTransfer as EtherTransferDto,
    Transfer as TransferDto,
};
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::Transfer as ServiceTransfer;
use crate::models::service::transactions::{
    Erc20Transfer, Erc721Transfer, EtherTransfer, TransactionInfo, TransactionStatus, TransferInfo,
};
use crate::providers::info::{
    InfoProvider, TokenInfo, TokenType,
};
use anyhow::Result;

impl TransferDto {
    pub fn to_transfer(&self, info_provider: &mut dyn InfoProvider, safe: &str) -> TransactionInfo {
        match self {
            TransferDto::Erc721(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction(info_provider, safe))
            }
            TransferDto::Erc20(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction(info_provider, safe))
            }
            TransferDto::Ether(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction(safe))
            }
            _ => TransactionInfo::Unknown,
        }
    }

    pub fn to_transaction_details(
        &self,
        info_provider: &mut dyn InfoProvider,
        safe: &String,
    ) -> Result<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: self.get_execution_time(),
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transfer(info_provider, safe),
            tx_data: None,
            tx_hash: self.get_transaction_hash(),
            detailed_execution_info: None,
        })
    }

    pub(super) fn get_execution_time(&self) -> Option<i64> {
        match self {
            TransferDto::Erc721(transfer) => Some(transfer.execution_date.timestamp_millis()),
            TransferDto::Erc20(transfer) => Some(transfer.execution_date.timestamp_millis()),
            TransferDto::Ether(transfer) => Some(transfer.execution_date.timestamp_millis()),
            _ => None,
        }
    }

    pub(super) fn get_transaction_hash(&self) -> Option<String> {
        match self {
            TransferDto::Erc721(transfer) => Some(transfer.transaction_hash.to_owned()),
            TransferDto::Erc20(transfer) => Some(transfer.transaction_hash.to_owned()),
            TransferDto::Ether(transfer) => Some(transfer.transaction_hash.to_owned()),
            _ => None,
        }
    }
}

impl Erc20TransferDto {
    pub(super) fn to_transfer_transaction(&self, info_provider: &mut dyn InfoProvider, safe: &str) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            direction: get_transfer_direction(safe, &self.from, &self.to),
            transfer_info: self.to_transfer_info(info_provider),
        }
    }

    pub(super) fn to_transfer_info(&self, info_provider: &mut dyn InfoProvider) -> TransferInfo {
        let token_info = self.get_token_info(info_provider);
        let info_ref = token_info.as_ref();
        TransferInfo::Erc20(Erc20Transfer {
            token_address: self.token_address.clone(),
            value: self.value.clone(),
            token_name: info_ref.map(|it| it.name.to_owned()),
            decimals: info_ref.map(|it| it.decimals.to_owned()),
            logo_uri: info_ref.and_then(|it| it.logo_uri.to_owned()),
            token_symbol: info_ref.map(|it| it.symbol.to_owned()),
        })
    }

    pub(super) fn get_token_info(&self, info_provider: &mut dyn InfoProvider) -> Option<Erc20TokenInfo> {
        token_info_with_fallback(
            info_provider,
            &self.token_address,
            self.token_info.clone(),
            TokenType::Erc20,
            |token| Erc20TokenInfo {
                address: token.address.to_owned(),
                name: token.name.to_owned(),
                symbol: token.symbol.to_owned(),
                decimals: token.decimals,
                logo_uri: token.logo_uri.to_owned(),
            },
        )
    }
}

impl Erc721TransferDto {
    pub(super) fn to_transfer_transaction(&self, info_provider: &mut dyn InfoProvider, safe: &str) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            direction: get_transfer_direction(safe, &self.from, &self.to),
            transfer_info: self.to_transfer_info(info_provider),
        }
    }

    pub(super) fn to_transfer_info(&self, info_provider: &mut dyn InfoProvider) -> TransferInfo {
        let token_info = self.get_token_info(info_provider);
        let info_ref = token_info.as_ref();
        TransferInfo::Erc721(Erc721Transfer {
            token_address: self.token_address.clone(),
            token_id: self.token_id.clone(),
            token_name: info_ref.map(|it| it.name.to_owned()),
            token_symbol: info_ref.map(|it| it.symbol.to_owned()),
            logo_uri: info_ref.and_then(|it| it.logo_uri.to_owned()),
        })
    }

    pub(super) fn get_token_info(&self, info_provider: &mut dyn InfoProvider) -> Option<Erc721TokenInfo> {
        token_info_with_fallback(
            info_provider,
            &self.token_address,
            self.token_info.clone(),
            TokenType::Erc721,
            |token| Erc721TokenInfo {
                name: token.name.to_owned(),
                symbol: token.symbol.to_owned(),
                logo_uri: token.logo_uri.to_owned(),
            },
        )
    }
}

impl EtherTransferDto {
    pub(super) fn to_transfer_transaction(&self, safe: &str) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            direction: get_transfer_direction(safe, &self.from, &self.to),
            transfer_info: self.to_transfer_info(),
        }
    }

    pub(super) fn to_transfer_info(&self) -> TransferInfo {
        TransferInfo::Ether(EtherTransfer {
            value: self.value.clone(),
        })
    }
}

fn token_info_with_fallback<T>(
    info_provider: &mut dyn InfoProvider,
    token_address: &String,
    token_info: Option<T>,
    expected_type: TokenType,
    fallback_mapper: impl Fn(&TokenInfo) -> T,
) -> Option<T> {
    if token_info.is_some() { return token_info; }
    match info_provider.token_info(token_address) {
        Ok(token) => {
            if token.token_type == expected_type {
                Some(fallback_mapper(&token))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
