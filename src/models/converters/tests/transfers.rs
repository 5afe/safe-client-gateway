use crate::models::backend::transfers::{
    Erc20Transfer as Erc20TransferDto, Erc721Transfer as Erc721TransferDto,
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::models::service::addresses::AddressEx;
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::{
    Erc20Transfer, Erc721Transfer, NativeCoinTransfer, TransactionInfo, TransactionStatus,
    Transfer, TransferDirection, TransferInfo,
};
use crate::providers::info::*;

#[rocket::async_test]
async fn erc_20_transfer_dto_to_transaction_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc_20_transfer = TransferDto::Erc20(
        serde_json::from_str::<Erc20TransferDto>(
            crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING,
        )
        .unwrap(),
    );

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_add_address_info_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F"),
        recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        direction: TransferDirection::Incoming,
        transfer_info: TransferInfo::Erc20(
            Erc20Transfer {
                token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
                value: "1000000000000000000".to_string(),
                token_name: Some("Dai".to_string()),
                token_symbol: Some("DAI".to_string()),
                decimals: Some(18),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
            }
        ),
    });

    let actual = erc_20_transfer
        .to_transfer(&mut mock_info_provider, safe_address)
        .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc_721_transfer_dto_to_transaction_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc_721_transfer = TransferDto::Erc721(
        serde_json::from_str::<Erc721TransferDto>(
            crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
        )
        .unwrap(),
    );

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_add_address_info_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0x938bae50a210b80EA233112800Cd5Bc2e7644300"),
        recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        direction: TransferDirection::Incoming,
        transfer_info: TransferInfo::Erc721(
            Erc721Transfer {
                token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
                token_id: "37".to_string(),
                token_name: Some("PV Memorial Token".to_string()),
                token_symbol: Some("PVT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
            }
        ),
    });

    let actual = erc_721_transfer
        .to_transfer(&mut mock_info_provider, safe_address)
        .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn ether_transfer_dto_to_transaction_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap(),
    );

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_add_address_info_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F"),
        recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        direction: TransferDirection::Incoming,
        transfer_info: (TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "1000000000000000".to_string(),
        })),
    });

    let actual = ether_transfer_dto
        .to_transfer(&mut mock_info_provider, safe_address)
        .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn unknown_transfer_dto_to_transaction_info() {
    let unknown_transfer_dto = TransferDto::Unknown;
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider.expect_add_address_info_from_contract_info().times(0);

    let actual = unknown_transfer_dto
        .to_transfer(&mut mock_info_provider, safe_address)
        .await;

    assert_eq!(TransactionInfo::Unknown, actual);
}

#[test]
fn unknown_transfer_dto_get_execution_time() {
    let unknown_transfer_dto = TransferDto::Unknown;

    let actual = unknown_transfer_dto.get_execution_time();

    assert_eq!(None, actual);
}

#[test]
fn unknown_transfer_dto_get_transaction_hash() {
    let unknown_transfer_dto = TransferDto::Unknown;

    let actual = unknown_transfer_dto.get_transaction_hash();

    assert_eq!(None, actual);
}

#[rocket::async_test]
async fn transfer_dto_to_transaction_details() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let ether_transfer_dto = TransferDto::Ether(
        serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER_INCOMING).unwrap(),
    );

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_add_address_info_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionDetails {
        executed_at: Some(1597733631000),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F"),
            recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
            direction: TransferDirection::Incoming,
            transfer_info: (TransferInfo::NativeCoin(NativeCoinTransfer {
                value: "1000000000000000".to_string(),
            })),
        }),
        tx_data: None,
        detailed_execution_info: None,
        tx_hash: Some(
            "0x41b610e8cce50bbe3aa06d6953ecc5f92a838aedc024a265c0afca7ec4f33bdf".to_string(),
        ),
        safe_app_info: None,
    };

    let actual = ether_transfer_dto
        .to_transaction_details(&mut mock_info_provider, safe_address)
        .await
        .unwrap();

    assert_eq!(expected, actual)
}

#[rocket::async_test]
async fn transfer_erc20_transfer_with_erc721_token_info_returns_transfer_tx() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_add_address_info_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let erc_20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_ERC721_TOKEN_INFO,
    )
    .unwrap();

    let transfer = TransferDto::Erc20(erc_20_transfer);
    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0xd31e655bC4Eb5BCFe25A47d636B25bb4aa4041B2"),
        recipient: AddressEx::address_only("0xBc79855178842FDBA0c353494895DEEf509E26bB"),
        direction: TransferDirection::Incoming,
        transfer_info: TransferInfo::Erc721(Erc721Transfer {
            token_address: "0xa9517B2E61a57350D6555665292dBC632C76adFe".to_string(),
            token_id: "856420144564".to_string(),
            token_name: Some("a!NEVER VISIT www.168pools.com to check DeFi ROi !".to_string()),
            token_symbol: Some("a!NEVER VISIT www.168pools.com to check DeFi ROi !".to_string()),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xa9517B2E61a57350D6555665292dBC632C76adFe.png".to_string()),
        }),
    });

    let actual = transfer
        .to_transfer(
            &mut mock_info_provider,
            "0xBc79855178842FDBA0c353494895DEEf509E26bB",
        )
        .await;

    assert_eq!(expected, actual)
}
