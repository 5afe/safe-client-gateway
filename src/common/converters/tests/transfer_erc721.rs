use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::transfers::{
    Erc721Transfer as Erc721TransferDto, Transfer as TransferDto,
};
use crate::providers::info::*;
use crate::routes::transactions::models::TransferInfo;
use crate::routes::transactions::models::{Erc721Transfer, Transfer, TransferDirection};

#[rocket::async_test]
async fn erc721_transfer_dto_to_incoming_transfer_transaction() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = Transfer {
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
    };

    let actual = Erc721TransferDto::to_transfer_transaction(
        &erc721_transfer,
        &mut mock_info_provider,
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_incoming_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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

    let expected = Transfer {
        sender: AddressEx { value: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(), name: Some("".to_string()), logo_uri: None },
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
    };

    let actual = Erc721TransferDto::to_transfer_transaction(
        &erc721_transfer,
        &mut mock_info_provider,
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_outgoing_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_OUTGOING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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

    let expected = Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx{ value: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(), name: Some("".to_string()), logo_uri: None },
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::Erc721(
            Erc721Transfer {
                token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
                token_id: "37".to_string(),
                token_name: Some("PV Memorial Token".to_string()),
                token_symbol: Some("PVT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
            }
        ),
    };

    let actual = Erc721TransferDto::to_transfer_transaction(
        &erc721_transfer,
        &mut mock_info_provider,
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_transfer_info_token_available() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TransferInfo::Erc721(
        Erc721Transfer {
            token_id: "37".to_string(),
            token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
            token_name: Some("PV Memorial Token".to_string()),
            token_symbol: Some("PVT".to_string()),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
        }
    );

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_transfer_info_token_unavailable() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| bail!("No token info"));

    let expected = TransferInfo::Erc721(Erc721Transfer {
        token_id: "37".to_string(),
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_name: None,
        token_symbol: None,
        logo_uri: None,
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_token_info_present() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TransferInfo::Erc721(Erc721Transfer{
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: Some("PV Memorial Token".to_string()),
        token_symbol: Some("PVT".to_string()),
        logo_uri:  Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string())
    }) ;

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_token_info_not_present() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO,
    )
    .unwrap();
    let token_info =
        serde_json::from_str::<TokenInfo>(crate::tests::json::TOKEN_PV_MEMORIAL_TOKEN).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TransferInfo::Erc721 (Erc721Transfer{
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: Some("PV Memorial Token".to_string()),
        token_symbol: Some("PVT".to_string()),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_info_provider_error() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::tests::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| bail!("No token info"));

    let expected = TransferInfo::Erc721(Erc721Transfer {
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: None,
        token_symbol: None,
        logo_uri: None,
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[test]
fn erc721_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc721(
        serde_json::from_str::<Erc721TransferDto>(
            crate::tests::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO,
        )
        .unwrap(),
    );

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1595594051000), actual);
}

#[test]
fn erc721_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc721(
        serde_json::from_str::<Erc721TransferDto>(
            crate::tests::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO,
        )
        .unwrap(),
    );

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(
        Some("0x6b4ddfcf19320e1edaad5bcdef3da54f463ee5cb609ba4a1e2042fbff702e718".to_string()),
        actual
    );
}
