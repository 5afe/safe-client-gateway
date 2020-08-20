use crate::models::backend::transfers::{Transfer as TransferDto, Erc20Transfer};

#[test]
fn erc20_transfer_dto_to_transfer_transaction() {}

#[test]
fn erc20_transfer_dto_to_transfer_info() {}

#[test]
fn erc20_transfer_dto_get_token_info_available() {}

#[test]
fn erc20_transfer_dto_get_token_info_unavailable() {}


#[test]
fn erc20_transfer_dto_get_execution_time() {
    let ether_transfer_dto = TransferDto::Erc20(serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER).unwrap());

    let actual = TransferDto::get_execution_time(&ether_transfer_dto);

    assert_eq!(Some(1597162353000), actual);
}


#[test]
fn erc20_transfer_dto_get_transaction_hash() {
    let ether_transfer_dto = TransferDto::Erc20(serde_json::from_str::<Erc20Transfer>(crate::json::ERC_20_TRANSFER).unwrap());

    let actual = TransferDto::get_transaction_hash(&ether_transfer_dto);

    assert_eq!(Some("0x3663ae11e5414620b0fd7fe7c8175e4356070a0a403e6e6516d7aece29b7680d".to_string()), actual);
}
