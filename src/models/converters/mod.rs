pub mod balances;
pub mod chains;
pub mod data_decoded;
pub mod page_metadata;
pub mod safes;
pub mod transactions;
pub mod transfers;

#[cfg(test)]
mod tests;

use crate::models::service::transactions::TransferDirection;
use crate::providers::address_info::AddressInfo;
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
pub(super) async fn get_address_info(
    safe: &str,
    address: &str,
    info_provider: &impl InfoProvider,
) -> Option<AddressInfo> {
    if safe != address {
        info_provider.full_address_info_search(address).await.ok()
    } else {
        None
    }
}
