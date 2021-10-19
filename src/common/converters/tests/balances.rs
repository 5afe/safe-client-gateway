use crate::common::models::backend::balances::Balance as BalanceDto;
use crate::common::models::backend::chains::NativeCurrency;
use crate::json::{BALANCE_COMPOUND_ETHER, BALANCE_ETHER};
use crate::providers::info::{TokenInfo, TokenType};
use crate::routes::balances::models::Balance;

#[test]
fn native_token_balance() {
    std::env::set_var("FEATURE_FLAG_BALANCES_RATE_IMPLEMENTATION", "false");
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::NativeToken,
            address: "0x0000000000000000000000000000000000000000".to_string(),
            decimals: 18,
            symbol: "ETH".to_string(),
            name: "Ether".to_string(),
            logo_uri: Some("https://test.token.image.url".to_string()),
        },
        balance: "7457594371050000001".to_string(),
        fiat_balance: "2523.7991".to_string(),
        fiat_conversion: "338.42".to_string(),
    };

    let usd_to_fiat = 1.0;
    let native_currency = NativeCurrency {
        name: "Ether".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18,
        logo_uri: "https://test.token.image.url".to_string(),
    };
    let actual = balance_dto.to_balance(usd_to_fiat, &native_currency);

    assert_eq!(actual, expected);
}

#[test]
fn erc20_token_balance_usd_balance() {
    std::env::set_var("FEATURE_FLAG_BALANCES_RATE_IMPLEMENTATION", "false");
    std::env::set_var("VPC_TRANSACTION_SERVICE_URI", "false");
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_COMPOUND_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::Erc20,
            address: "0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e".to_string(),
            decimals: 8,
            symbol: "cETH".to_string(),
            name: "Compound Ether 📈".to_string(),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e.png".to_string()),
        },
        balance: "5002".to_string(),
        fiat_balance: "0.0014".to_string(),
        fiat_conversion: "28.5462".to_string(),
    };

    let usd_to_fiat = 1.0;
    let native_currency = NativeCurrency {
        name: "Compound Ether 📈".to_string(),
        symbol: "cETH".to_string(),
        decimals: 8,
        logo_uri: "https://test.token.image.url".to_string(),
    };
    let actual = balance_dto.to_balance(usd_to_fiat, &native_currency);

    assert_eq!(actual, expected);
}

#[test]
fn erc20_token_balance_fiat_is_twice_usd() {
    std::env::set_var("FEATURE_FLAG_BALANCES_RATE_IMPLEMENTATION", "false");
    std::env::set_var("VPC_TRANSACTION_SERVICE_URI", "false");
    let balance_dto = serde_json::from_str::<BalanceDto>(BALANCE_COMPOUND_ETHER).unwrap();

    let expected = Balance {
        token_info: TokenInfo {
            token_type: TokenType::Erc20,
            address: "0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e".to_string(),
            decimals: 8,
            symbol: "cETH".to_string(),
            name: "Compound Ether 📈".to_string(),
            logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xd6801a1DfFCd0a410336Ef88DeF4320D6DF1883e.png".to_string()),
        },
        balance: "5002".to_string(),
        fiat_balance: "0.0028".to_string(),
        fiat_conversion: "57.0924".to_string(),
    };

    let usd_to_fiat = 2.0;
    let native_currency = NativeCurrency {
        name: "Compound Ether 📈".to_string(),
        symbol: "cETH".to_string(),
        decimals: 8,
        logo_uri: "https://test.token.image.url".to_string(),
    };
    let actual = balance_dto.to_balance(usd_to_fiat, &native_currency);

    assert_eq!(actual, expected);
}
