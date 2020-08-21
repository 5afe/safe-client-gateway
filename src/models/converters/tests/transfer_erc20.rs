use crate::models::backend::transfers::{Transfer as TransferDto, Erc20Transfer, Erc20TokenInfo};
use crate::providers::info::*;

#[test]
fn erc20_transfer_dto_to_transfer_transaction() {}

#[test]
fn erc20_transfer_dto_to_transfer_info() {}

#[test]
fn erc20_transfer_dto_get_token_info_present() {
    let erc20_transfer = serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);

    let expected = Erc20TokenInfo {
        address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        name: "Dai".to_string(),
        symbol: "DAI".to_string(),
        decimals: 18,
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
    };
    let actual = Erc20Transfer::get_token_info(&erc20_transfer, &mut mock_info_provider).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn erc20_transfer_dto_get_token_info_not_present() {
    let erc20_transfer = serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO).unwrap();
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_DAI).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = Erc20TokenInfo {
        address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
        name: "Dai".to_string(),
        symbol: "DAI".to_string(),
        decimals: 18,
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
    };

    let actual = Erc20Transfer::get_token_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn erc20_transfer_dto_get_info_provider_error() {
    let erc20_transfer = serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();

    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));

    let actual = Erc20Transfer::get_token_info(&erc20_transfer, &mut mock_info_provider);

    assert_eq!(None, actual);
}

#[test]
fn erc20_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc20(serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO).unwrap());

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597162353000), actual);
}


#[test]
fn erc20_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc20(serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER_WITHOUT_TOKEN_INFO).unwrap());

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(Some("0x3663ae11e5414620b0fd7fe7c8175e4356070a0a403e6e6516d7aece29b7680d".to_string()), actual);
}
