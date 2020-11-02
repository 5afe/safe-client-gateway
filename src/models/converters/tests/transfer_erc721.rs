use crate::models::backend::transfers::{
    Erc721Transfer as Erc721TransferDto, Transfer as TransferDto,
};
use crate::models::service::transactions::TransferInfo;
use crate::models::service::transactions::{Erc721Transfer, Transfer, TransferDirection};
use crate::providers::info::*;

#[test]
fn erc721_transfer_dto_to_transfer_transaction() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = Transfer {
        sender: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
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
    );

    assert_eq!(expected, actual);
}

#[test]
fn erc721_transfer_dto_to_transfer_info_token_available() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO)
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

    let actual = Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc721_transfer_dto_to_transfer_info_token_unavailable() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));

    let expected = TransferInfo::Erc721(Erc721Transfer {
        token_id: "37".to_string(),
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_name: None,
        token_symbol: None,
        logo_uri: None,
    });

    let actual = Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn erc721_transfer_dto_get_token_info_present() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TokenInfo {
        token_type: TokenType::Erc721,
        address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        name: "PV Memorial Token".to_string(),
        symbol: "PVT".to_string(),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
        decimals: 0
    };
    let actual =
        Erc721TransferDto::get_token_info(&erc721_transfer, &mut mock_info_provider).unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn erc721_transfer_dto_get_token_info_not_present() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let token_info =
        serde_json::from_str::<TokenInfo>(crate::json::TOKEN_PV_MEMORIAL_TOKEN).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TokenInfo {
        token_type: TokenType::Erc721,
        address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        name: "PV Memorial Token".to_string(),
        symbol: "PVT".to_string(),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
        decimals: 0
    };

    let actual = Erc721TransferDto::get_token_info(&erc721_transfer, &mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn erc721_transfer_dto_get_info_provider_error() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| anyhow::bail!("No token info"));

    let actual = Erc721TransferDto::get_token_info(&erc721_transfer, &mut mock_info_provider);

    assert_eq!(None, actual);
}

#[test]
fn erc721_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc721(
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap(),
    );

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1595594051000), actual);
}

#[test]
fn erc721_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc721(
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap(),
    );

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(
        Some("0x6b4ddfcf19320e1edaad5bcdef3da54f463ee5cb609ba4a1e2042fbff702e718".to_string()),
        actual
    );
}
