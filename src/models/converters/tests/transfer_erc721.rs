use crate::models::backend::transfers::{
    Erc721Transfer as Erc721TransferDto, Transfer as TransferDto,
};
use crate::models::chains::{ChainInfo, NativeCurrency};
use crate::models::service::transactions::TransferInfo;
use crate::models::service::transactions::{Erc721Transfer, Transfer, TransferDirection};
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;

#[rocket::async_test]
async fn erc721_transfer_dto_to_incoming_transfer_transaction() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_full_address_info_search()
        .times(1)
        .return_once(move |_, _| bail!("No address info"));
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

    let expected = Transfer {
        sender: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
        sender_info: None,
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
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
        "4",
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_incoming_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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

    let expected = Transfer {
        sender: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
        sender_info: Some(AddressInfo{ name: "".to_string(), logo_uri: None }),
        recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient_info: None,
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
        "4",
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_outgoing_transfer_transaction_with_address_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_OUTGOING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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

    let expected = Transfer {
        sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        sender_info: None,
        recipient: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
        recipient_info: Some(AddressInfo{ name: "".to_string(), logo_uri: None }),
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
        "4",
        safe_address,
    )
    .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_transfer_info_token_available() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider, "4").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_to_transfer_info_token_unavailable() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_, _| bail!("No token info"));
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

    let expected = TransferInfo::Erc721(Erc721Transfer {
        token_id: "37".to_string(),
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_name: None,
        token_symbol: None,
        logo_uri: None,
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider, "4").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_token_info_present() {
    let erc721_transfer = serde_json::from_str::<Erc721TransferDto>(
        crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING,
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
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

    let expected = TransferInfo::Erc721(Erc721Transfer{
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: Some("PV Memorial Token".to_string()),
        token_symbol: Some("PVT".to_string()),
        logo_uri:  Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string())
    }) ;

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider, "4").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_token_info_not_present() {
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
        .return_once(move |_, _| Ok(token_info));
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

    let expected = TransferInfo::Erc721 (Erc721Transfer{
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: Some("PV Memorial Token".to_string()),
        token_symbol: Some("PVT".to_string()),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider, "4").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn erc721_transfer_dto_get_info_provider_error() {
    let erc721_transfer =
        serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITHOUT_TOKEN_INFO)
            .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_, _| bail!("No token info"));
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

    let expected = TransferInfo::Erc721(Erc721Transfer {
        token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
        token_id: "37".to_string(),
        token_name: None,
        token_symbol: None,
        logo_uri: None,
    });

    let actual =
        Erc721TransferDto::to_transfer_info(&erc721_transfer, &mut mock_info_provider, "4").await;

    assert_eq!(expected, actual);
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
