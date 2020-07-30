use crate::models::service::transactions::{TransferInfo, TransactionInfo};
use crate::models::service::transactions::Transfer as ServiceTransfer;
use crate::models::backend::transfers::{Transfer as TransferDto, Erc20Transfer, Erc721Transfer, EtherTransfer};

impl TransferDto {
    pub fn to_transfer(&self) -> TransactionInfo {
        match self {
            TransferDto::Erc721(transfer) => TransactionInfo::Transfer(transfer.to_transfer_transaction()),
            TransferDto::Erc20(transfer) => TransactionInfo::Transfer(transfer.to_transfer_transaction()),
            TransferDto::Ether(transfer) => TransactionInfo::Transfer(transfer.to_transfer_transaction()),
            _ => TransactionInfo::Unknown
        }
    }
}

impl Erc20Transfer {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: TransferInfo::Erc20 {
                token_address: self.token_address.clone(),
                value: self.value.clone(),
                token_name: self.token_info.as_ref().map(|it| it.name.to_owned()),
                decimals: self.token_info.as_ref().map(|it| it.decimals.to_owned()),
                logo_uri: self.token_info.as_ref().and_then(|it| it.logo_uri.to_owned()),
                token_symbol: self.token_info.as_ref().map(|it| it.symbol.to_owned()),
            },
        }
    }
}

impl Erc721Transfer {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: TransferInfo::Erc721 {
                token_address: self.token_address.clone(),
                token_id: self.token_id.clone(),
                token_name: self.token_info.as_ref().map(|it| it.name.to_owned()),
                token_symbol: self.token_info.as_ref().map(|it| it.symbol.to_owned()),
                logo_uri: self.token_info.as_ref().and_then(|it| it.logo_uri.to_owned()),
            },
        }
    }
}

impl EtherTransfer {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from.to_owned(),
            recipient: self.to.to_owned(),
            transfer_info: TransferInfo::Ether {
                value: self.value.clone()
            },
        }
    }
}