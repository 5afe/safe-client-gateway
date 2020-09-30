use crate::models::service::balances::Balance;
use crate::utils::errors::ApiResult;

pub fn balances_usd(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![])
}

pub fn balances_euro(safe_address: &str) -> ApiResult<Vec<Balance>> {
    Ok(vec![])
}