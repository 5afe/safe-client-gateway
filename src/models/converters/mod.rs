pub mod transactions;
pub mod transfers;
pub mod data_decoded;

#[cfg(test)]
mod tests;

use crate::models::service::transactions::TransferDirection;

fn get_transfer_direction(safe: &String, from: &String, to: &String) -> TransferDirection {
    if safe == from {
        TransferDirection::Outgoing
    } else if safe == to {
        TransferDirection::Incoming
    } else {
        TransferDirection::Unknown
    }
}