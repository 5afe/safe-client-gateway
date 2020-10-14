use crate::models::backend::transactions::ModuleTransaction;
use crate::models::backend::requests::ConfirmationRequest as BackendConfirmationRequest;
use crate::models::service::transactions::requests::ConfirmationRequest;
use crate::models::service::transactions::details::{TransactionDetails, DetailedExecutionInfo, MultisigExecutionDetails, TransactionData};
use crate::models::service::transactions::{TransactionStatus, TransactionInfo};
use crate::models::commons::Operation;

#[test]
#[should_panic]
fn build_confirmation_wrong_detailed_execution_type() {
    let module_transaction = serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX).unwrap();
    let confirmation_request = ConfirmationRequest {
        signer: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
    };
    let tx_details = module_transaction.to_transaction_details().unwrap();
    let actual = confirmation_request.build_confirmation_request("safe_address", "safe_tx_hash", tx_details);

    actual.unwrap();
}

#[test]
#[should_panic]
fn build_confirmation_tx_data_is_none() {
    let transaction_details = TransactionDetails {
        executed_at: None,
        tx_status: TransactionStatus::AwaitingConfirmations,
        tx_info: TransactionInfo::Unknown,
        tx_data: None,
        detailed_execution_info: Some(DetailedExecutionInfo::Multisig(MultisigExecutionDetails {
            submitted_at: 0,
            nonce: 0,
            safe_tx_gas: 0,
            base_gas: 0,
            gas_price: "".to_string(),
            gas_token: "".to_string(),
            refund_receiver: "".to_string(),
            safe_tx_hash: "".to_string(),
            executor: None,
            signers: vec![],
            confirmations_required: 0,
            confirmations: vec![],
            gas_token_info: None,
        })),
        tx_hash: None,
    };
    let confirmation_request = ConfirmationRequest {
        signer: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
    };
    let actual = confirmation_request.build_confirmation_request("safe_address", "safe_tx_hash", transaction_details);

    actual.unwrap();
}


#[test]
fn build_confirmation_tx_data_value_and_data_are_none() {
    let transaction_details = TransactionDetails {
        executed_at: None,
        tx_status: TransactionStatus::AwaitingConfirmations,
        tx_info: TransactionInfo::Unknown,
        tx_data: Some(TransactionData {
            hex_data: None,
            data_decoded: None,
            to: "".to_string(),
            value: None,
            operation: Operation::CALL,
        }),
        detailed_execution_info: Some(DetailedExecutionInfo::Multisig(MultisigExecutionDetails {
            submitted_at: 0,
            nonce: 0,
            safe_tx_gas: 0,
            base_gas: 0,
            gas_price: "".to_string(),
            gas_token: "".to_string(),
            refund_receiver: "".to_string(),
            safe_tx_hash: "".to_string(),
            executor: None,
            signers: vec![],
            confirmations_required: 0,
            confirmations: vec![],
            gas_token_info: None,
        })),
        tx_hash: None,
    };
    let confirmation_request = ConfirmationRequest {
        signer: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
    };
    let expected = BackendConfirmationRequest {
        safe: "safe_address".to_string(),
        to: "".to_string(),
        value: "0".to_string(),
        data: "0x".to_string(),
        operation: "0".to_string(),
        gas_token: "".to_string(),
        safe_tx_gas: "0".to_string(),
        base_gas: "0".to_string(),
        gas_price: "".to_string(),
        refund_receiver: "".to_string(),
        nonce: "0".to_string(),
        contract_transaction_hash: "safe_tx_hash".to_string(),
        sender: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
        origin: "".to_string(),
    };
    let actual = confirmation_request.build_confirmation_request("safe_address", "safe_tx_hash", transaction_details).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn build_confirmation() {
    let transaction_details = TransactionDetails {
        executed_at: None,
        tx_status: TransactionStatus::AwaitingConfirmations,
        tx_info: TransactionInfo::Unknown,
        tx_data: Some(TransactionData {
            hex_data: Some("0x1234".to_string()),
            data_decoded: None,
            to: "to".to_string(),
            value: Some("30".to_string()),
            operation: Operation::CALL,
        }),
        detailed_execution_info: Some(
            DetailedExecutionInfo::Multisig(MultisigExecutionDetails {
                submitted_at: 0,
                nonce: 0,
                safe_tx_gas: 40,
                base_gas: 50,
                gas_price: "60".to_string(),
                gas_token: "gas_token".to_string(),
                refund_receiver: "refund_receiver".to_string(),
                safe_tx_hash: "".to_string(),
                executor: None,
                signers: vec![],
                confirmations_required: 0,
                confirmations: vec![],
                gas_token_info: None,
            })),
        tx_hash: None,
    };
    let confirmation_request = ConfirmationRequest {
        signer: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
    };
    let expected = BackendConfirmationRequest {
        safe: "safe_address".to_string(),
        to: "to".to_string(),
        value: "30".to_string(),
        data: "0x1234".to_string(),
        operation: "0".to_string(),
        gas_token: "gas_token".to_string(),
        safe_tx_gas: "40".to_string(),
        base_gas: "50".to_string(),
        gas_price: "60".to_string(),
        refund_receiver: "refund_receiver".to_string(),
        nonce: "0".to_string(),
        contract_transaction_hash: "safe_tx_hash".to_string(),
        sender: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        signature: "0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001".to_string(),
        origin: "".to_string(),
    };
    let actual = confirmation_request.build_confirmation_request("safe_address", "safe_tx_hash", transaction_details).unwrap();

    assert_eq!(expected, actual);
}