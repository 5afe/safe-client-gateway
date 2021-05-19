use crate::utils::transactions::{cancellation_parts_hash, domain_hash, hash};
use ethcontract_common::hash::keccak256;
use ethereum_types::Address;

#[test]
fn domain_hash_for_safe_address() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let actual = to_hex_string!(domain_hash(&safe_address, false).to_vec());
    assert_eq!(
        "0x6dda5da6f3b6225311946ab4732b5658018db6dc890378fbdb529d8e9832762a",
        actual
    );
}

#[test]
fn domain_hash_for_safe_address_legacy() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let actual = to_hex_string!(domain_hash(&safe_address, true).to_vec());
    assert_eq!(
        "0x6dda5da6f3b6225311946ab4732b5658018db6dc890378fbdb529d8e9832762a",
        actual
    );
}

#[test]
fn safe_tx_hash_for_safe_address_cancellation_tx() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let nonce = 39;

    let domain_hash = domain_hash(&safe_address, true);
    let actual = to_hex_string!(hash(safe_address, nonce, domain_hash).to_vec());
    assert_eq!(
        "0x89067bfebe450e45c02dd97e3cc9bd1656d49ebb8a17819829eab9c5dc575c27",
        actual
    );
}

#[test]
fn parts_hash_for_cancellation() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let nonce = 39;

    let actual = cancellation_parts_hash(&safe_address, nonce);
    assert_eq!(
        to_hex_string!(actual),
        "0xf0c66ea90dae4d21f8fed03cb6e7f03eb0720479fb2562915921721eed809626"
    );
}

#[test]
fn empty_data_keccak() {
    assert_eq!(
        to_hex_string!(keccak256(vec![])),
        "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
    );
}
