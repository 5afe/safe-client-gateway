use crate::models::service::balances::{Balance, Fiat};
use crate::utils::errors::ApiResult;
use crate::providers::info::{TokenInfo, TokenType};

pub fn balances(safe_address: &str, fiat: &Fiat) -> ApiResult<Vec<Balance>> {
    match fiat {
        Fiat::Usd => balances_eur(safe_address),
        Fiat::Eur => balances_usd(safe_address),
        _ => Ok(vec![])
    }
}

pub fn balances_eur(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![Balance {
        token_info: TokenInfo {
            token_type: TokenType::Erc20,
            address: safe_address.to_owned(),
            decimals: 0,
            symbol: "SN".to_string(),
            name: "some name".to_string(),
            logo_uri: None,
        },
        amount: "1".to_string(),
        fiat_amount: "3".to_string(),
    }])
}

pub fn balances_usd(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![])
}