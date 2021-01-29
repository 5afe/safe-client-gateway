use crate::models::backend::transfers::{
    Erc20Transfer as Erc20TransferDto, Transfer as TransferDto,
};
use crate::models::service::transactions::TransferInfo;
use crate::models::service::transactions::{Erc20Transfer, Transfer, TransferDirection};
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;

#[test]
fn erc20_transfer_dto_to_incoming_transfer_transaction() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No address info"));

    let expected = Transfer {
        sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        sender_info: None,
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
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
    };

    let actual = Erc20TransferDto::to_transfer_transaction(
        &erc20_transfer,
        &mut mock_info_provider,
        safe_address,
    );

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_to_incoming_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "".to_string(),
                logo_uri: None,
            })
        });

    let expected = Transfer {
        sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        sender_info: Some(AddressInfo{
            name: "".to_string(),
            logo_uri: None
        }),
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
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
    };

    let actual = Erc20TransferDto::to_transfer_transaction(
        &erc20_transfer,
        &mut mock_info_provider,
        safe_address,
    );

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_to_outgoing_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_OUTGOING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "".to_string(),
                logo_uri: None,
            })
        });

    let expected = Transfer {
        sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        sender_info: None,
        recipient: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
        recipient_info: Some(AddressInfo{
            name: "".to_string(),
            logo_uri: None
        }),
        direction: TransferDirection::Outgoing,
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
    };

    let actual = Erc20TransferDto::to_transfer_transaction(
        &erc20_transfer,
        &mut mock_info_provider,
        safe_address,
    );

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_to_transfer_info_token_available() {
    let erc20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TransferInfo::Erc20(
        Erc20Transfer {
            token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
            value: "1000000000000000000".to_string(),
            token_name: Some("Dai".to_string()),
            token_symbol: Some("DAI".to_string()),
            decimals: Some(18),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
        }
    );

    let actual = Erc20TransferDto::to_transfer_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_to_transfer_info_token_unavailable() {
    let erc20_transfer =
        serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));

    let expected = TransferInfo::Erc20(Erc20Transfer {
        token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        value: "1000000000000000000".to_string(),
        token_name: None,
        token_symbol: None,
        decimals: None,
        logo_uri: None,
    });

    let actual = Erc20TransferDto::to_transfer_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_get_token_info_present() {
    let erc20_transfer = serde_json::from_str::<Erc20TransferDto>(
        crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TransferInfo::Erc20 (Erc20Transfer{
        token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        token_name: Some("Dai".to_string()),
        token_symbol: Some("DAI".to_string()),
        decimals: Some(18),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
        value: "1000000000000000000".to_string()
    });
    let actual = Erc20TransferDto::to_transfer_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(actual, expected);
}

#[test]
fn erc20_transfer_dto_get_token_info_not_present() {
    let erc20_transfer =
        serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_DAI).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TransferInfo::Erc20 (Erc20Transfer{
        token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        token_name: Some("Dai".to_string()),
        token_symbol: Some("DAI".to_string()),
        decimals: Some(18),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
        value: "1000000000000000000".to_string()
    });

    let actual = Erc20TransferDto::to_transfer_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_get_info_provider_error() {
    let erc20_transfer =
        serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));
    let expected = TransferInfo::Erc20(Erc20Transfer {
        token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        token_name: None,
        token_symbol: None,
        decimals: None,
        logo_uri: None,
        value: "1000000000000000000".to_string(),
    });

    let actual = Erc20TransferDto::to_transfer_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc20_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc20(
        serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap(),
    );

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597162353000), actual);
}

#[test]
fn erc20_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc20(
        serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap(),
    );

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(
        Some("0x3663ae11e5414620b0fd7fe7c8175e4356070a0a403e6e6516d7aece29b7680d".to_string()),
        actual
    );
}
