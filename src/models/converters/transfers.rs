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
    InfoProvider, TokenInfo, TokenType, TokenType::Erc20, TokenType::Erc721,
};
use anyhow::Result;

impl TransferDto {
    pub fn to_transfer(&self, info_provider: &mut InfoProvider) -> TransactionInfo {
        match self {
            TransferDto::Erc721(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction(info_provider))
            }
            TransferDto::Erc20(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction(info_provider))
            }
            TransferDto::Ether(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction())
            }
            _ => TransactionInfo::Unknown,
        }
    }

    pub fn to_transaction_details(
        &self,
        info_provider: &mut InfoProvider,
    ) -> Result<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: self.get_execution_time(),
            submitted_at: None,
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transfer(info_provider),
            tx_data: None,
            tx_hash: self.get_transaction_hash(),
            detailed_execution_info: None,
        })
    }

    fn get_execution_time(&self) -> Option<i64> {
        match self {
            TransferDto::Erc721(transfer) => Some(transfer.execution_date.timestamp_millis()),
            TransferDto::Erc20(transfer) => Some(transfer.execution_date.timestamp_millis()),
            TransferDto::Ether(transfer) => Some(transfer.execution_date.timestamp_millis()),
            _ => None,
        }
    }

    fn get_transaction_hash(&self) -> Option<String> {
        match self {
            TransferDto::Erc721(transfer) => Some(transfer.transaction_hash.to_owned()),
            TransferDto::Erc20(transfer) => Some(transfer.transaction_hash.to_owned()),
            TransferDto::Ether(transfer) => Some(transfer.transaction_hash.to_owned()),
            _ => None,
        }
    }
}

impl Erc20TransferDto {
    fn to_transfer_transaction(&self, info_provider: &mut InfoProvider) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: self.to_transfer_info(info_provider),
        }
    }

    fn to_transfer_info(&self, info_provider: &mut InfoProvider) -> TransferInfo {
        let token_info = self.get_token_info(info_provider);
        TransferInfo::Erc20(Erc20Transfer {
            token_address: self.token_address.clone(),
            value: self.value.clone(),
            token_name: self.token_info.as_ref().map(|it| it.name.to_owned()),
            decimals: self.token_info.as_ref().map(|it| it.decimals.to_owned()),
            logo_uri: self
                .token_info
                .as_ref()
                .and_then(|it| it.logo_uri.to_owned()),
            token_symbol: self.token_info.as_ref().map(|it| it.symbol.to_owned()),
        })
    }

    fn get_token_info(&self, info_provider: &mut InfoProvider) -> Option<Erc20TokenInfo> {
        token_info_with_fallback(
            info_provider,
            &self.token_address,
            self.token_info,
            TokenType::Erc20,
            |token| Erc20TokenInfo {
                address: token.address,
                name: token.name,
                symbol: token.symbol,
                decimals: token.decimals,
                logo_uri: token.logo_uri,
            },
        )
    }
}

impl Erc721TransferDto {
    fn to_transfer_transaction(&self, info_provider: &mut InfoProvider) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: self.to_transfer_info(info_provider),
        }
    }

    fn to_transfer_info(&self, info_provider: &mut InfoProvider) -> TransferInfo {
        TransferInfo::Erc721(Erc721Transfer {
            token_address: self.token_address.clone(),
            token_id: self.token_id.clone(),
            token_name: self.token_info.as_ref().map(|it| it.name.to_owned()),
            token_symbol: self.token_info.as_ref().map(|it| it.symbol.to_owned()),
            logo_uri: self
                .token_info
                .as_ref()
                .and_then(|it| it.logo_uri.to_owned()),
        })
    }

    fn get_token_info(&self, info_provider: &mut InfoProvider) -> Option<Erc721TokenInfo> {
        token_info_with_fallback(
            info_provider,
            &self.token_address,
            self.token_info,
            TokenType::Erc20,
            |token| Erc721TokenInfo {
                name: token.name,
                symbol: token.symbol,
                logo_uri: token.logo_uri,
            },
        )
    }
}

impl EtherTransferDto {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: self.to_transfer_info(),
        }
    }

    fn to_transfer_info(&self) -> TransferInfo {
        TransferInfo::Ether(EtherTransfer {
            value: self.value.clone(),
        })
    }
}

fn token_info_with_fallback<T>(
    info_provider: &mut InfoProvider,
    token_address: &String,
    token_info: Option<T>,
    expected_type: TokenType,
    fallback_mapper: impl Fn(TokenInfo) -> T,
) -> Option<T> {
    if token_info.is_some() {
        return token_info;
    }
    match info_provider.token_info(token_address) {
        Ok(token) => match token.token_type {
            expected_type => Some(fallback_mapper(token)),
            _ => None,
        },
        Err(_) => None,
    }
}
