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
            sender: self.from,
            recipient: self.to,
            transfer_info: TransferInfo::Erc20 {
                token_name: self.token_info.name.clone(),
                token_symbol: self.token_info.symbol.clone(),
                logo_uri: self.token_info.logo_uri.clone(),
                decimals: self.token_info.decimals,
                value: self.value.clone(),
            },
        }
    }
}

impl Erc721Transfer {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from,
            recipient: self.to,
            transfer_info: TransferInfo::Erc721 {
                token_id: self.token_id.clone(),
                token_address: self.token_address.clone(),
            },
        }
    }
}

impl EtherTransfer {
    fn to_transfer_transaction(&self) -> ServiceTransfer {
        ServiceTransfer {
            sender: self.from,
            recipient: self.to,
            transfer_info: TransferInfo::Ether {
                value: self.value.clone()
            },
        }
    }
}