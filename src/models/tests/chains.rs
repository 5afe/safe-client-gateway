use crate::models::backend::chains::{ChainInfo, NativeCurrency, Theme};
use crate::models::service::chains::{
    ChainInfo as ServiceChainInfo, NativeCurrency as ServiceNativeCurrency, Theme as ServiceTheme,
};
use rocket::serde::json::json;

#[test]
fn chain_info_json() {
    let chain_info_json = json!({
          "chainId": "4",
          "chainName": "Rinkeby",
          "rpcUrl": "https://someurl.com/rpc",
          "blockExplorerUrl": "https://blockexplorer.com",
          "transactionService": "https://safe-transaction.rinkeby.staging.gnosisdev.com",
          "nativeCurrency": {
            "name": "Ether",
            "symbol": "ETH",
            "decimals": 18,
            "logoUrl": "https://test.token.image.url",
          },
          "theme": {
            "textColor": "#fff",
            "backgroundColor": "#000"
          },
          "ensRegistryAddress": "0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF",
          "recommendedMasterCopyVersion": "1.1.1"
    });

    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
    };

    let actual = serde_json::from_str::<ChainInfo>(&chain_info_json.to_string());

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_to_service_chain_info() {
    let chain_info_json = json!({
          "chainId": "4",
          "chainName": "Rinkeby",
          "rpcUrl": "https://someurl.com/rpc",
          "blockExplorerUrl": "https://blockexplorer.com",
          "transactionService": "https://safe-transaction.rinkeby.staging.gnosisdev.com",
          "nativeCurrency": {
            "name": "Ether",
            "symbol": "ETH",
            "decimals": 18,
            "logoUrl": "https://test.token.image.url",
          },
          "theme": {
            "textColor": "#fff",
            "backgroundColor": "#000"
          },
          "ensRegistryAddress": "0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF",
          "recommendedMasterCopyVersion": "1.1.1"
    });

    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
    };

    let from_json = serde_json::from_str::<ChainInfo>(&chain_info_json.to_string()).unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}
