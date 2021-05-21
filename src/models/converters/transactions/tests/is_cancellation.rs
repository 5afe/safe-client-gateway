use super::super::chrono::Utc;
use crate::models::backend::transactions::MultisigTransaction;
use crate::models::commons::Operation;

#[test]
fn is_cancellation_result_true() {
    let tx = build_multisig_tx();

    assert_eq!(true, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_0x_data_result_true() {
    let mut tx = build_multisig_tx();
    tx.data = Some(String::from("0x"));

    assert_eq!(true, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_safe_tx_gas_result_false() {
    let mut tx = build_multisig_tx();
    tx.safe_tx_gas = Some(1);

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_value_result_false() {
    let mut tx = build_multisig_tx();
    tx.value = Some(String::from("1"));

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_data_result_false() {
    let mut tx = build_multisig_tx();
    tx.data = Some(String::from("0x12345678"));

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_delegate_operation_false() {
    let mut tx = build_multisig_tx();
    tx.operation = Operation::DELEGATE;

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_base_gas_result_false() {
    let mut tx = build_multisig_tx();
    tx.base_gas = Some(1);

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_gas_price_result_false() {
    let mut tx = build_multisig_tx();
    tx.gas_price = Some(String::from("1"));

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_gas_token_result_false() {
    let mut tx = build_multisig_tx();
    tx.gas_token = Some(String::from("0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46"));

    assert_eq!(false, tx.is_cancellation());
}

#[test]
fn is_cancellation_has_refund_receiver_result_false() {
    let mut tx = build_multisig_tx();
    tx.refund_receiver = Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"));

    assert_eq!(false, tx.is_cancellation());
}

fn build_multisig_tx() -> MultisigTransaction {
    MultisigTransaction {
        safe: "0x1".to_string(),
        to: "0x1".to_string(),
        value: None,
        data: None,
        data_decoded: None,
        operation: Operation::CALL,
        gas_token: None,
        safe_tx_gas: None,
        base_gas: None,
        gas_price: None,
        refund_receiver: None,
        nonce: 0,
        execution_date: None,
        submission_date: Utc::now(),
        modified: None,
        block_number: None,
        transaction_hash: None,
        safe_tx_hash: "".to_string(),
        executor: None,
        is_executed: false,
        is_successful: None,
        eth_gas_price: None,
        gas_used: None,
        fee: None,
        origin: None,
        confirmations_required: None,
        confirmations: None,
        signatures: None,
    }
}
