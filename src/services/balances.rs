use crate::models::service::balances::Balance;
use crate::utils::errors::ApiResult;

pub fn balances(safe_address: &str, fiat: &str) -> ApiResult<Vec<Balance>> {
    match fiat {
        "eur" => balances_eur(safe_address),
        "usd" => balances_usd(safe_address),
        _ => Ok(vec![])
    }
}

pub fn balances_eur(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![])
}

pub fn balances_usd(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![])
}