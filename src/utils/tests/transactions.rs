use crate::utils::transactions::{domain_hash, hash};
use ethereum_types::Address;

#[test]
fn domain_hash_for_safe_address() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let actual = to_hex_string!(domain_hash(&safe_address).to_vec());
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

    let actual = to_hex_string!(hash(&safe_address, nonce).to_vec());
    assert_eq!(
        "0x931e3e46c1c06ad4449ae193d159dab9e24c50112682ffea083e0052ba53900b",
        actual
    );
}
