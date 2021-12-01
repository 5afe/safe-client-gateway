use crate::common::models::backend::chains::{
    BlockExplorerUriTemplate, ChainInfo, GasPrice, NativeCurrency, RpcAuthentication, RpcUri, Theme,
};
use crate::routes::chains::models::{
    BlockExplorerUriTemplate as ServiceBlockExplorerUriTemplate, ChainInfo as ServiceChainInfo,
    GasPrice as ServiceGasPrice, NativeCurrency as ServiceNativeCurrency,
    RpcAuthentication as ServiceRpcAuthentication, RpcUri as ServiceRpcUri, Theme as ServiceTheme,
};

#[test]
fn chain_info_json() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![GasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual = serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_fixed_gas_price() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![GasPrice::Fixed {
            wei_value: "1000000000".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_FIXED_GAS_PRICE);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_no_gas_price() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_NO_GAS_PRICE);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_multiple_gas_price() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![
            GasPrice::Oracle {
                uri: "https://gaspriceoracle.com/".to_string(),
                gas_parameter: "average".to_string(),
                gwei_factor: "10".to_string(),
            },
            GasPrice::Fixed {
                wei_value: "1000000000".to_string(),
            },
        ],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual = serde_json::from_str::<ChainInfo>(
        crate::tests::json::CHAIN_INFO_RINKEBY_MULTIPLE_GAS_PRICE,
    );

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_unknown_gas_price_type() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![GasPrice::Unknown],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_UNKNOWN_GAS_PRICE);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_no_rpc_authentication() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::NoAuthentication,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![GasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual = serde_json::from_str::<ChainInfo>(
        crate::tests::json::CHAIN_INFO_RINKEBY_RPC_NO_AUTHENTICATION,
    );

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_unknown_rpc_authentication() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        vpc_transaction_service: "http://rinkeby-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::Unknown,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![GasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let actual = serde_json::from_str::<ChainInfo>(
        crate::tests::json::CHAIN_INFO_RINKEBY_RPC_UNKNOWN_AUTHENTICATION,
    );

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let from_json =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY).unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn unknown_gas_price_type_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Unknown],
        disabled_wallets: vec![],
        features: vec![],
    };

    let from_json =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_UNKNOWN_GAS_PRICE)
            .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn no_authentication_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::NoAuthentication,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let from_json = serde_json::from_str::<ChainInfo>(
        crate::tests::json::CHAIN_INFO_RINKEBY_RPC_NO_AUTHENTICATION,
    )
    .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn unknown_authentication_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::Unknown,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    };

    let from_json = serde_json::from_str::<ChainInfo>(
        crate::tests::json::CHAIN_INFO_RINKEBY_RPC_UNKNOWN_AUTHENTICATION,
    )
    .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn disabled_wallets_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![String::from("metamask"), String::from("trezor")],
        features: vec![],
    };

    let from_json =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_DISABLED_WALLETS)
            .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn features_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        short_name: "rin".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc".to_string(),
        },
        safe_apps_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/apps".to_string(),
        },
        public_rpc_uri: ServiceRpcUri {
            authentication: ServiceRpcAuthentication::ApiKeyPath,
            value: "https://someurl.com/rpc/public".to_string(),
        },
        block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
            address: "https://blockexplorer.com/{{address}}".to_string(),
            tx_hash: "https://blockexplorer.com/{{txHash}}".to_string(),
            api: "https://blockexplorer.com/api".to_string(),
        },
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: vec![ServiceGasPrice::Oracle {
            uri: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![String::from("Feature 1"), String::from("Feature 2")],
    };

    let from_json =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY_ENABLED_FEATURES)
            .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}
