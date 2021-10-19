use crate::common::converters::get_transfer_direction;
use crate::routes::transactions::models::TransferDirection;

#[test]
fn get_transfer_direction_incoming() {
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let to = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let from = "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0";

    let actual = get_transfer_direction(safe, from, to);

    assert_eq!(actual, TransferDirection::Incoming);
}

#[test]
fn get_transfer_direction_outgoing() {
    let safe = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let to = "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0";
    let from = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";

    let actual = get_transfer_direction(safe, from, to);

    assert_eq!(actual, TransferDirection::Outgoing);
}

#[test]
fn get_transfer_direction_unknown() {
    let safe = "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23";
    let to = "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0";
    let from = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";

    let actual = get_transfer_direction(safe, from, to);

    assert_eq!(actual, TransferDirection::Unknown);
}
