use crate::models::backend::transfers::{
    Erc20Transfer as Erc20TransferDto, Erc721Transfer as Erc721TransferDto,
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::Transfer as ServiceTransfer;
use crate::models::service::transactions::{
    Erc20Transfer, Erc721Transfer, EtherTransfer, TransactionInfo, TransactionStatus, TransferInfo,
};
use anyhow::Result;

impl TransferDto {
    pub fn to_transfer(&self) -> TransactionInfo {
        match self {
            TransferDto::Erc721(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction())
            }
            TransferDto::Erc20(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction())
            }
            TransferDto::Ether(transfer) => {
                TransactionInfo::Transfer(transfer.to_transfer_transaction())
            }
            _ => TransactionInfo::Unknown,
        }
    }

    pub fn to_transaction_details(&self) -> Result<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: self.get_execution_time(),
            submitted_at: None,
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transfer(),
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
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: self.to_transfer_info(),
        }
    }

    fn to_transfer_info(&self) -> TransferInfo {
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
}

impl Erc721TransferDto {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: self.to_transfer_info(),
        }
    }

    fn to_transfer_info(&self) -> TransferInfo {
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
