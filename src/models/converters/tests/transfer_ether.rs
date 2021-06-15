use crate::models::backend::transfers::{
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::models::chains::{ChainInfo, NativeCurrency};
use crate::models::service::transactions::{
    EtherTransfer, Transfer, TransferDirection, TransferInfo,
};
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;

#[rocket::async_test]
async fn ether_transfer_dto_ether_incoming_transfer_transaction() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .times(1)
        .return_once(move |_, _| bail!("no address info"));
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(|_| {
            Ok(ChainInfo {
                transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com"
                    .to_string(),
                chain_id: "4".to_string(),
                chain_name: "Rinkeby".to_string(),
                rpc_url: "some_url".to_string(),
                block_explorer_url: "some_url".to_string(),
                native_currency: NativeCurrency {
                    name: "Ether".to_string(),
                    symbol: "ETH".to_string(),
                    decimals: 18,
                },
            })
        });

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        sender_info: None,
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::Ether(EtherTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        "4",
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn ether_transfer_dto_ether_incoming_transfer_transaction_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .times(1)
        .return_once(move |_, _| {
            Ok(AddressInfo {
                name: "".to_string(),
                logo_uri: None,
            })
        });
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(|_| {
            Ok(ChainInfo {
                transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com"
                    .to_string(),
                chain_id: "4".to_string(),
                chain_name: "Rinkeby".to_string(),
                rpc_url: "some_url".to_string(),
                block_explorer_url: "some_url".to_string(),
                native_currency: NativeCurrency {
                    name: "Ether".to_string(),
                    symbol: "ETH".to_string(),
                    decimals: 18,
                },
            })
        });

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        sender_info: Some(AddressInfo {
            name: "".to_string(),
            logo_uri: None,
        }),
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::Ether(EtherTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        "4",
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn ether_transfer_dto_ether_outgoing_transfer_transaction_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .times(1)
        .return_once(move |_, _| {
            Ok(AddressInfo {
                name: "".to_string(),
                logo_uri: None,
            })
        });
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(|_| {
            Ok(ChainInfo {
                transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com"
                    .to_string(),
                chain_id: "4".to_string(),
                chain_name: "Rinkeby".to_string(),
                rpc_url: "some_url".to_string(),
                block_explorer_url: "some_url".to_string(),
                native_currency: NativeCurrency {
                    name: "Ether".to_string(),
                    symbol: "ETH".to_string(),
                    decimals: 18,
                },
            })
        });

    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_OUTGOING).unwrap();
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Transfer {
        sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        sender_info: None,
        recipient: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        recipient_info: Some(AddressInfo {
            name: "".to_string(),
            logo_uri: None,
        }),
        direction: TransferDirection::Outgoing,
        transfer_info: (TransferInfo::Ether(EtherTransfer {
            value: "1000000000000000".to_string(),
        })),
    };

    let actual = EtherTransferDto::to_transfer_transaction(
        &ether_transfer_dto,
        &mut mock_info_provider,
        "4",
        safe,
    )
    .await;

    assert_eq!(expected, actual);
}

#[test]
fn ether_transfer_dto_to_transfer_info() {
    let ether_transfer_dto =
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap();
    let expected = TransferInfo::Ether(EtherTransfer {
        value: "1000000000000000".to_string(),
    });

    let actual = EtherTransferDto::to_transfer_info(&ether_transfer_dto);

    assert_eq!(expected, actual);
}

#[test]
fn ether_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap(),
    );

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597733631000), actual);
}

#[test]
fn ether_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap(),
    );

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(
        Some("0x41b610e8cce50bbe3aa06d6953ecc5f92a838aedc024a265c0afca7ec4f33bdf".to_string()),
        actual
    );
}
