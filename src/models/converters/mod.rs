pub mod transactions;
pub mod transfers;
pub mod data_decoded;

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
