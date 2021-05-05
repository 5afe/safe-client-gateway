use super::get_transfer_direction;
use crate::models::backend::transfers::{
    Erc20Transfer as Erc20TransferDto, Erc721Transfer as Erc721TransferDto,
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::models::converters::get_address_info;
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::Transfer as ServiceTransfer;
use crate::models::service::transactions::{
    Erc20Transfer, Erc721Transfer, EtherTransfer, TransactionInfo, TransactionStatus, TransferInfo,
};
use crate::providers::info::{InfoProvider, TokenInfo, TokenType};
use crate::utils::errors::ApiResult;

impl TransferDto {
    pub async fn to_transfer(
        &self,
        info_provider: &impl InfoProvider,
        safe: &str,
    ) -> TransactionInfo {
        match self {
            TransferDto::Erc721(transfer) => TransactionInfo::Transfer(
                transfer.to_transfer_transaction(info_provider, safe).await,
            ),
            TransferDto::Erc20(transfer) => TransactionInfo::Transfer(
                transfer.to_transfer_transaction(info_provider, safe).await,
            ),
            TransferDto::Ether(transfer) => TransactionInfo::Transfer(
                transfer.to_transfer_transaction(info_provider, safe).await,
            ),
            _ => TransactionInfo::Unknown,
        }
    }

    pub async fn to_transaction_details(
        &self,
        info_provider: &impl InfoProvider,
        safe: &str,
    ) -> ApiResult<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: self.get_execution_time(),
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transfer(info_provider, safe).await,
            tx_data: None,
            tx_hash: self.get_transaction_hash(),
            detailed_execution_info: None,
            safe_app_info: None,
            address_info_index: None,
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
    pub(super) async fn to_transfer_transaction(
        &self,
        info_provider: &impl InfoProvider,
        safe: &str,
    ) -> ServiceTransfer {
        ServiceTransfer {
            sender_info: get_address_info(safe, &self.from, info_provider).await,
            sender: self.from.to_owned(),
            recipient_info: get_address_info(safe, &self.to, info_provider).await,
            recipient: self.to.to_owned(),
            direction: get_transfer_direction(safe, &self.from, &self.to),
            transfer_info: self.to_transfer_info(info_provider).await,
        }
    }

    pub(super) async fn to_transfer_info(&self, info_provider: &impl InfoProvider) -> TransferInfo {
        let token_info =
            token_info_with_fallback(info_provider, &self.token_address, self.token_info.clone())
                .await;
        build_transfer_info(
            token_info.as_ref(),
            TokenType::Erc20,
            &self.token_address,
            &self.value,
        )
    }
}

impl Erc721TransferDto {
    pub(super) async fn to_transfer_transaction(
        &self,
        info_provider: &impl InfoProvider,
        safe: &str,
    ) -> ServiceTransfer {
        ServiceTransfer {
            sender_info: get_address_info(safe, &self.from, info_provider).await,
            sender: self.from.to_owned(),
            recipient_info: get_address_info(safe, &self.to, info_provider).await,
            recipient: self.to.to_owned(),
            direction: get_transfer_direction(safe, &self.from, &self.to),
            transfer_info: self.to_transfer_info(info_provider).await,
        }
    }

    pub(super) async fn to_transfer_info(&self, info_provider: &impl InfoProvider) -> TransferInfo {
        let token_info =
            token_info_with_fallback(info_provider, &self.token_address, self.token_info.clone())
                .await;
        build_transfer_info(
            token_info.as_ref(),
            TokenType::Erc721,
            &self.token_address,
            &self.token_id,
        )
    }
}

impl EtherTransferDto {
    pub(super) async fn to_transfer_transaction(
        &self,
        info_provider: &impl InfoProvider,
        safe: &str,
    ) -> ServiceTransfer {
        ServiceTransfer {
            sender_info: get_address_info(safe, &self.from, info_provider).await,
            sender: self.from.to_owned(),
            recipient_info: get_address_info(safe, &self.to, info_provider).await,
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

fn build_transfer_info(
    token_info: Option<&TokenInfo>,
    default_token_type: TokenType,
    token_address: &str,
    element: &str,
) -> TransferInfo {
    match token_info
        .map(|it| it.token_type.to_owned())
        .unwrap_or(default_token_type)
    {
        TokenType::Erc20 => TransferInfo::Erc20(Erc20Transfer {
            token_address: token_address.to_owned(),
            token_name: token_info.map(|it| it.name.to_owned()),
            token_symbol: token_info.map(|it| it.symbol.to_owned()),
            logo_uri: token_info.map(|it| it.logo_uri.to_owned()).flatten(),
            decimals: token_info.map(|it| it.decimals.to_owned()),
            value: element.to_owned(),
        }),
        TokenType::Erc721 => TransferInfo::Erc721(Erc721Transfer {
            token_address: token_address.to_owned(),
            token_id: element.to_owned(),
            token_name: token_info.map(|it| it.name.to_owned()),
            token_symbol: token_info.map(|it| it.symbol.to_owned()),
            logo_uri: token_info.map(|it| it.logo_uri.to_owned()).flatten(),
        }),
        _ => panic!("Transfer token type not supported"),
    }
}

async fn token_info_with_fallback(
    info_provider: &impl InfoProvider,
    token_address: &str,
    token_info: Option<TokenInfo>,
) -> Option<TokenInfo> {
    if token_info.is_none() {
        info_provider.token_info(token_address).await.ok()
    } else {
        token_info
    }
}
