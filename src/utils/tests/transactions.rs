use crate::utils::transactions::{
    cancellation_parts_hash, domain_hash_v100, domain_hash_v130, hash, use_legacy_domain_separator,
};
use ethcontract_common::hash::keccak256;
use ethereum_types::Address;
use semver::Version;
use std::env;

#[test]
fn domain_hash_for_safe_address() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let actual = to_hex_string!(domain_hash_v130("4", &safe_address).to_vec()); // Rinkeby
    assert_eq!(
        "0x0d56532a2a780ffd32b2c3d85d0f8a7b2fc13df0576c006e2aaa47eb66cf71c9",
        actual
    );
}

#[test]
fn domain_hash_for_safe_address_legacy() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let actual = to_hex_string!(domain_hash_v100(&safe_address).to_vec());
    assert_eq!(
        "0x6dda5da6f3b6225311946ab4732b5658018db6dc890378fbdb529d8e9832762a",
        actual
    );
}

#[test]
fn safe_tx_hash_for_safe_address_cancellation_tx_legacy() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),
    ))
    .unwrap();
    let nonce = 39;
    let domain_hash = domain_hash_v100(&safe_address);

    let actual = to_hex_string!(hash(safe_address, nonce, domain_hash).to_vec());
    assert_eq!(
        "0x89067bfebe450e45c02dd97e3cc9bd1656d49ebb8a17819829eab9c5dc575c27",
        actual
    );
}

#[test]
fn safe_tx_hash_for_safe_address_cancellation_tx() {
    let safe_address: Address = serde_json::from_value(serde_json::value::Value::String(
        "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f".to_string(),
    ))
    .unwrap();
    let nonce = 39;
    let domain_hash = domain_hash_v130("4", &safe_address); // Rinkeby

    let actual = to_hex_string!(hash(safe_address, nonce, domain_hash).to_vec());
    assert_eq!(
        "0xdce3bf453ed8cf84d13c76911e5d11c31501b24004b9e856d6091808067bd398",
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

#[test]
fn use_legacy_domain_separator_v130() {
    let version = Version::parse("1.3.0").ok();

    assert_eq!(false, use_legacy_domain_separator(version));
}

#[test]
fn use_legacy_domain_separator_legacy() {
    let version = Version::parse("1.1.1").ok();

    assert_eq!(true, use_legacy_domain_separator(version));
}
