pub mod balances;
pub mod balances_v2;
pub mod chains;
pub mod data_decoded;
pub mod page_metadata;
pub mod safe_app;
pub mod safes;
pub mod transactions;
pub mod transfers;

#[cfg(test)]
mod tests;

use crate::models::service::addresses::AddressEx;
use crate::models::service::transactions::TransferDirection;
use crate::providers::info::InfoProvider;

pub(super) fn get_transfer_direction(safe: &str, from: &str, to: &str) -> TransferDirection {
    if safe == from {
        TransferDirection::Outgoing
    } else if safe == to {
        TransferDirection::Incoming
    } else {
        TransferDirection::Unknown
    }
}

// This method is required to prevent polluting the cache with all the safe requests
// This is done to prevent that every user that queries a transfer transaction, doesn't
// leave a mark in our cache.
pub(super) async fn get_address_ex_from_any_source(
    safe: &str,
    address: &str,
    info_provider: &impl InfoProvider,
) -> AddressEx {
    if safe != address {
        info_provider
            .address_ex_from_any_source(address)
            .await
            .unwrap_or(AddressEx::address_only(address))
    } else {
        AddressEx::address_only(address)
    }
}
