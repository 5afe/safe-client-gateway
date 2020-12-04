pub mod balances;
pub mod data_decoded;
pub mod page_metadata;
pub mod transactions;
pub mod transfers;

#[cfg(test)]
mod tests;

use crate::models::service::transactions::TransferDirection;

pub(super) fn get_transfer_direction(safe: &str, from: &str, to: &str) -> TransferDirection {
    if safe == from {
        TransferDirection::Outgoing
    } else if safe == to {
        TransferDirection::Incoming
    } else {
        TransferDirection::Unknown
    }
}
