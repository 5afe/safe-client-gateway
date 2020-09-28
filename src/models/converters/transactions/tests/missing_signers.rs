use crate::models::backend::transactions::MultisigTransaction;
use crate::providers::info::SafeInfo;

#[test]
fn missing_signers_on_awaiting_confirmation_empty() {
    // This Safe has 5 owners and the transaction has 2 signatures
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_AWAITING_CONFIRMATIONS_EMPTY).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    
    let actual = tx.missing_signers(&safe_info.owners);
    let expected: Vec<String> = vec![
        "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_owned(),
        "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_owned(),
        "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_owned(),
        "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_owned(),
        "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_owned()
      ];
    assert_eq!(expected, actual);
}

#[test]
fn missing_signers_on_awaiting_confirmation_null() {
    // This Safe has 5 owners and the transaction has 2 signatures
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_AWAITING_CONFIRMATIONS_NULL).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    
    let actual = tx.missing_signers(&safe_info.owners);
    let expected: Vec<String> = vec![
        "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_owned(),
        "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_owned(),
        "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_owned(),
        "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_owned(),
        "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_owned()
      ];
    assert_eq!(expected, actual);
}

#[test]
fn missing_signers_on_awaiting_confirmation() {
    // This Safe has 5 owners and the transaction has 1 signatures
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_AWAITING_CONFIRMATIONS).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();

    let actual = tx.missing_signers(&safe_info.owners);
    let expected: Vec<String> = vec![
        "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_owned(),
        "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_owned(),
        "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_owned(),
        "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_owned()
      ];
    assert_eq!(expected, actual);
}