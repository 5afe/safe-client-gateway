use crate::models::backend::transfers::EtherTransfer as EtherTransferDto;
use crate::models::service::transactions::{Transfer, TransferDirection, TransferInfo, EtherTransfer};

#[test]
fn ethe_transfer_backend_ether_transfer_service() {
    let ether_transfer_dto = serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::Ether(EtherTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(&ether_transfer_dto, safe);

    assert_eq!(expected, actual);
}

#[test]
fn transfer_dto_unknown_to_transaction_info() {}

#[test]
fn transfer_dto_erc20_to_transaction_info() {}

#[test]
fn transfer_dto_erc721_to_transaction_info() {}

#[test]
fn transfer_dto_ether_to_transaction_info() {}