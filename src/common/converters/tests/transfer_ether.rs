use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::transfers::{
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::providers::info::*;
use crate::routes::transactions::models::{
    NativeCoinTransfer, Transfer, TransferDirection, TransferInfo,
};

#[rocket::async_test]
async fn ether_transfer_dto_ether_incoming_transfer_transaction() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("no address info"));

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_INCOMING)
            .unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: AddressEx::address_only("0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F"),
        recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn ether_transfer_dto_ether_incoming_transfer_transaction_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some("".to_string()),
                logo_uri: None,
            })
        });

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_INCOMING)
            .unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: AddressEx {
            value: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
            name: Some("".to_string()),
            logo_uri: None,
        },
        recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn ether_transfer_dto_ether_outgoing_transfer_transaction_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some("".to_string()),
                logo_uri: None,
            })
        });

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_OUTGOING)
            .unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx {
            value: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
            name: Some("".to_string()),
            logo_uri: None,
        },
        direction: TransferDirection::Outgoing,
        transfer_info: (TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[test]
fn ether_transfer_dto_to_transfer_info() {
    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_INCOMING)
            .unwrap();
    let expected = TransferInfo::NativeCoin(NativeCoinTransfer {
        value: "1000000000000000".to_string(),
    });

    let actual = EtherTransferDto::to_transfer_info(&ether_transfer_dto);

    assert_eq!(expected, actual);
}

#[test]
fn ether_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_INCOMING)
            .unwrap(),
    );

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597733631000), actual);
}

#[test]
fn ether_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::tests::json::ETHER_TRANSFER_INCOMING)
            .unwrap(),
    );

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(
        Some("0x41b610e8cce50bbe3aa06d6953ecc5f92a838aedc024a265c0afca7ec4f33bdf".to_string()),
        actual
    );
}
