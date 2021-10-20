pub mod data_decoded;
pub mod page_metadata;
pub mod transfers;

#[cfg(test)]
mod tests;

use crate::common::models::addresses::AddressEx;
use crate::providers::info::InfoProvider;
use crate::routes::transactions::models::TransferDirection;

pub(crate) fn get_transfer_direction(safe: &str, from: &str, to: &str) -> TransferDirection {
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
pub(crate) async fn get_address_ex_from_any_source(
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
