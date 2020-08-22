use crate::models::backend::transfers::{EtherTransfer as EtherTransferDto, Transfer as TransferDto};
use crate::models::service::transactions::{Transfer, TransferDirection, TransferInfo, EtherTransfer};

#[test]
fn ether_transfer_dto_ether_transfer_transaction() {
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
fn ether_transfer_dto_to_transfer_info() {
    let ether_transfer_dto = serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap();
    let expected = TransferInfo::Ether(EtherTransfer {
            value: "1000000000000000".to_string(),
        });

    let actual = EtherTransferDto::to_transfer_info(&ether_transfer_dto);

    assert_eq!(expected, actual);
}

#[test]
fn ether_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Ether(serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap());

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597733631000), actual);
}

#[test]
fn ether_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Ether(serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap());

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(Some("0x41b610e8cce50bbe3aa06d6953ecc5f92a838aedc024a265c0afca7ec4f33bdf".to_string()), actual);
}
