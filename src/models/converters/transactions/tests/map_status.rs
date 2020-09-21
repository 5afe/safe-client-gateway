use crate::models::backend::transactions::MultisigTransaction;
use crate::providers::info::SafeInfo;
use crate::models::service::transactions::TransactionStatus;

#[test]
pub (super) fn map_status_to_awaiting_confirmation() {
    let tx_json = crate::json::MULTISIG_TX_ETHER_TRANSFER;
    let safe_info_json = crate::json::SAFE_WITH_MODULES;
    let tx = serde_json::from_str::<MultisigTransaction>(tx_json).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(safe_info_json).unwrap();
    let actual = tx.map_status(&safe_info);

    assert_eq!(TransactionStatus::Success, actual);
}