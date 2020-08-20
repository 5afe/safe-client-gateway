use crate::models::backend::transfers::{Transfer as TransferDto, Erc721Transfer};

#[test]
fn erc721_transfer_dto_to_transfer_transaction() {}

#[test]
fn erc721_transfer_dto_to_transfer_info() {}

#[test]
fn erc721_transfer_dto_get_token_info_available() {}

#[test]
fn erc721_transfer_dto_get_token_info_unavailable() {}

#[test]
fn erc721_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc721(serde_json::from_str::<Erc721Transfer>(crate::json::ERC_721_TRANSFER).unwrap());

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(Some("0x6b4ddfcf19320e1edaad5bcdef3da54f463ee5cb609ba4a1e2042fbff702e718".to_string()), actual);
}

#[test]
fn erc721_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc721(serde_json::from_str::<Erc721Transfer>(crate::json::ERC_721_TRANSFER).unwrap());

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1595594051000), actual);
}
