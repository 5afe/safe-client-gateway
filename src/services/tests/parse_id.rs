use crate::models::service::transactions::TransactionIdParts;
use crate::services::transactions_details::parse_id;

#[test]
fn multisig_details_id() {
    let details_id = "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x8bde30060a1e4d8383efa9b666654b31771c93325f905088c91f58803b4433b5";
    let expected = TransactionIdParts::Multisig {
        safe_address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        safe_tx_hash: "0x8bde30060a1e4d8383efa9b666654b31771c93325f905088c91f58803b4433b5".to_string(),
    };

    let actual = parse_id(details_id).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn ethereum_details_id() {
    let details_id = "ethereum_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x4071662b18fb425db9a516b8472b4f545decb4bb6f6873af098b123b544e3cf4_0xae2714c8d2239062";
    let expected = TransactionIdParts::Ethereum {
        safe_address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        transaction_hash: "0x4071662b18fb425db9a516b8472b4f545decb4bb6f6873af098b123b544e3cf4".to_string(),
        details_hash: "0xae2714c8d2239062".to_string(),
    };

    let actual = parse_id(details_id).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn module_details_id() {
    let details_id = "module_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x4071662b18fb425db9a516b8472b4f545decb4bb6f6873af098b123b544e3cf4_0xae2714c8d2239062";
    let expected = TransactionIdParts::Module {
        safe_address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        transaction_hash: "0x4071662b18fb425db9a516b8472b4f545decb4bb6f6873af098b123b544e3cf4".to_string(),
        details_hash: "0xae2714c8d2239062".to_string(),
    };

    let actual = parse_id(details_id).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn transaction_hash_details_id() {
    let details_id = "0x8bde30060a1e4d8383efa9b666654b31771c93325f905088c91f58803b4433b5";
    let expected = TransactionIdParts::TransactionHash(String::from("0x8bde30060a1e4d8383efa9b666654b31771c93325f905088c91f58803b4433b5"));

    let actual = parse_id(details_id).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn creation_details_id() {
    let details_id = "creation_0x83eC7B0506556a7749306D69681aDbDbd08f0769";
    let expected = TransactionIdParts::Creation(String::from("0x83eC7B0506556a7749306D69681aDbDbd08f0769"));

    let actual = parse_id(details_id).unwrap();

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn failure_details_id() {
    let malformed_details_id = "module_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x4071662b18fb425db9a516b8472b4f545decb4bb6f6873af098b123b544e3cf4";

    parse_id(malformed_details_id).unwrap();
}