use crate::models::backend::transactions::MultisigTransaction;
use crate::providers::info::SafeInfo;
use crate::models::service::transactions::TransactionStatus;

#[test]
fn map_status_to_success() {
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ETHER_TRANSFER).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let actual = tx.map_status(&safe_info);

    assert_eq!(TransactionStatus::Success, actual);
}

#[test]
fn map_status_to_failed() {
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_FAILED_TX).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let actual = tx.map_status(&safe_info);

    assert_eq!(TransactionStatus::Failed, actual);
}

#[test]
fn map_status_to_cancelled() {
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC721_TRANSFER_CANCELLED).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES_AND_HIGH_NONCE).unwrap();
    let actual = tx.map_status(&safe_info);

    assert_eq!(TransactionStatus::Cancelled, actual);
}