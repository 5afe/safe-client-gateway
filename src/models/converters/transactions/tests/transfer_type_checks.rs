use crate::models::backend::transactions::{MultisigTransaction, Confirmation};
use crate::models::commons::{DataDecoded, Parameter};
use crate::providers::info::*;
use chrono::Utc;
use crate::models::commons::Operation;


#[test]
fn multisig_tx_check_erc721_transfer() {
    let expected_date = Utc::now();
    let safe = String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    let to = String::from("0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF");
    let multisig_tx = MultisigTransaction {
        safe,
        to,
        value: Some(String::from("0")),
        data: Some(String::from("0xa9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e76443000000000000000000000000000000000000000000000000000000000000000466")),
        data_decoded: Some(DataDecoded {
            data_decoded: None,
            method: String::from("transfer"),
            parameters: Some(vec!(
                Parameter { name: String::from("to"), param_type: String::from("address"), value: String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300").into() },
                Parameter { name: String::from("value"), param_type: String::from("uint256"), value: String::from("1126").into() },
            )),
        }),
        operation: Some(Operation::CALL),
        gas_token: Some(String::from("0x0000000000000000000000000000000000000000")),
        safe_tx_gas: Some(47810),
        base_gas: Some(0),
        gas_price: Some(String::from("0")),
        refund_receiver: Some(String::from("0x0000000000000000000000000000000000000000")),
        nonce: 175,
        execution_date: Some(expected_date),
        submission_date: expected_date,
        modified: Some(expected_date),
        block_number: Some(6954173),
        transaction_hash: Some(String::from("0xc76ee22b0ab2785c5c8e93d029acf5643a8fccabcc3f223704bdbfc2af8193ae")),
        safe_tx_hash: String::from("0xcddc60c644e85ee3bff84204380a86578d3b62f8dd05a86de40e7e662012caf5"),
        executor: Some(String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd")),
        is_executed: true,
        is_successful: Some(true),
        eth_gas_price: Some(String::from("1000000000")),
        gas_used: Some(80507),
        fee: Some(String::from("80507000000000")),
        origin: None,
        confirmations_required: Some(3),
        confirmations: Some(vec!(
            Confirmation {
                owner: String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x6722a6772c15dd72851593c69e521729be5f6e86cbe6881498885b6efb02f6a61638976cf12bb113d8a1f7cea776e9d5c4949dde7d8015ca03e433acc6f9435b1b")),
            },
            Confirmation {
                owner: String::from("0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x75aa624315ed8a72e1fcf396fe8d7a50d6d71f18c3b6bddb025bf426b7e4de3f52ead25cef15c9b35feaeefb2b81f2b5333ed949035f948fbe0a9aec56f7ac5b1b")),
            },
            Confirmation {
                owner: String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001")),
            },
        )),
        signatures: Some(String::from("0x6722a6772c15dd72851593c69e521729be5f6e86cbe6881498885b6efb02f6a61638976cf12bb113d8a1f7cea776e9d5c4949dde7d8015ca03e433acc6f9435b1b75aa624315ed8a72e1fcf396fe8d7a50d6d71f18c3b6bddb025bf426b7e4de3f52ead25cef15c9b35feaeefb2b81f2b5333ed949035f948fbe0a9aec56f7ac5b1b000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001")),
    };
    let token_info = Some(TokenInfo {
        token_type: TokenType::Erc721,
        address: String::from("0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF"),
        name: String::from("CryptoKitties"),
        symbol: String::from("CK"),
        decimals: 0,
        logo_uri: Some(String::from("https://gnosis-safe-token-logos.s3.amazonaws.com/0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF.png")),
    });
    assert!(!multisig_tx.is_erc20_transfer(&token_info));
    assert!(!multisig_tx.is_ether_transfer());
    assert!(multisig_tx.is_erc721_transfer(&token_info));
}

#[test]
fn multisig_tx_check_erc20_transfer() {
    let expected_date = Utc::now();
    let safe = String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    let to = String::from("0xF9bA5210F91D0474bd1e1DcDAeC4C58E359AaD85");
    let multisig_tx = MultisigTransaction {
        safe,
        to,
        value: Some(String::from("0")),
        data: Some(String::from("0xa9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000002d79883d2000")),
        data_decoded: Some(DataDecoded {
            data_decoded: None,
            method: String::from("transfer"),
            parameters: Some(vec!(
                Parameter { name: String::from("to"), param_type: String::from("address"), value: String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300").into() },
                Parameter { name: String::from("value"), param_type: String::from("uint256"), value: String::from("50000000000000").into() },
            )),
        }),
        operation: Some(Operation::CALL),
        gas_token: Some(String::from("0x0000000000000000000000000000000000000000")),
        safe_tx_gas: Some(36698),
        base_gas: Some(0),
        gas_price: Some(String::from("0")),
        refund_receiver: Some(String::from("0x0000000000000000000000000000000000000000")),
        nonce: 174,
        execution_date: Some(expected_date),
        submission_date: expected_date,
        modified: Some(expected_date),
        block_number: Some(6953959),
        transaction_hash: Some(String::from("0x8bdce717cbf13200180e418138cbf7325642fa8ff715c7a262328853b7792271")),
        safe_tx_hash: String::from("0xf94fc3d3607845b3e04e2c6ae51802ffe2b4d39dd9b3159a8d9d9e31a05b5beb"),
        executor: Some(String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0")),
        is_executed: true,
        is_successful: Some(true),
        eth_gas_price: Some(String::from("1000000000")),
        gas_used: Some(69431),
        fee: Some(String::from("69431000000000")),
        origin: None,
        confirmations_required: Some(3),
        confirmations: Some(vec!(
            Confirmation {
                owner: String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x00000000000000000000000065f8236309e5a99ff0d129d04e486ebce20dc7b0000000000000000000000000000000000000000000000000000000000000000001")),
            },
            Confirmation {
                owner: String::from("0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0xccbf629ad44c41c39e397d4c3c199593be908edf57d728204c2ebd5f384ad5c90f1e7fca86dab291e406724b99d7d956acaa01cb78d146e410a7accd1ffe0df01b")),
            },
            Confirmation {
                owner: String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x9b3733a7018e90ca4ebb1504e1c230260f8d41d0542180e915fa10af694f18b360f5fe6c9dc252e257b3d45071b32977224122165ff771a018a392bb62dbf1491b")),
            },
        )),
        signatures: Some(String::from("0x00000000000000000000000065f8236309e5a99ff0d129d04e486ebce20dc7b0000000000000000000000000000000000000000000000000000000000000000001ccbf629ad44c41c39e397d4c3c199593be908edf57d728204c2ebd5f384ad5c90f1e7fca86dab291e406724b99d7d956acaa01cb78d146e410a7accd1ffe0df01b9b3733a7018e90ca4ebb1504e1c230260f8d41d0542180e915fa10af694f18b360f5fe6c9dc252e257b3d45071b32977224122165ff771a018a392bb62dbf1491b")),
    };
    let token_info = Some(TokenInfo {
        token_type: TokenType::Erc20,
        address: String::from("0xF9bA5210F91D0474bd1e1DcDAeC4C58E359AaD85"),
        name: String::from("Maker"),
        symbol: String::from("MKR"),
        decimals: 18,
        logo_uri: Some(String::from("https://gnosis-safe-token-logos.s3.amazonaws.com/0xF9bA5210F91D0474bd1e1DcDAeC4C58E359AaD85.png")),
    });
    assert!(multisig_tx.is_erc20_transfer(&token_info));
    assert!(!multisig_tx.is_ether_transfer());
    assert!(!multisig_tx.is_erc721_transfer(&token_info));
}

#[test]
fn multisig_tx_check_ether_transfer() {
    let expected_date = Utc::now();
    let safe = String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300");
    let to = String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0");
    let multisig_tx = MultisigTransaction {
        safe,
        to,
        value: Some(String::from("50000000000000")),
        data: None,
        data_decoded: None,
        operation: Some(Operation::CALL),
        gas_token: Some(String::from("0x0000000000000000000000000000000000000000")),
        safe_tx_gas: Some(27845),
        base_gas: Some(0),
        gas_price: Some(String::from("0")),
        refund_receiver: Some(String::from("0x0000000000000000000000000000000000000000")),
        nonce: 47,
        execution_date: Some(expected_date),
        submission_date: expected_date,
        modified: Some(expected_date),
        block_number: Some(6971215),
        transaction_hash: Some(String::from("0x26aa6abf9d8945d108f7c45e833f4d5b506a1cf3251d3feec2161bcb5f9fe62a")),
        safe_tx_hash: String::from("0xc09dd34d20c4edbce12b18a5fd49a075c95fd5dc95235321689d903487854129"),
        executor: Some(String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd")),
        is_executed: true,
        is_successful: Some(true),
        eth_gas_price: Some(String::from("1000000000")),
        gas_used: Some(69431),
        fee: Some(String::from("53493000000000")),
        origin: None,
        confirmations_required: Some(2),
        confirmations: Some(vec!(
            Confirmation {
                owner: String::from("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("EOA"),
                signature: Some(String::from("0x8b841f4b1b24c4db687bed9d9754253f94cd543d05447b31d32d945be6967a636a454fb210e058f783115c84fd141adf0e8d2fedb17df19419858cbb03fdddb31c")),
            },
            Confirmation {
                owner: String::from("0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"),
                submission_date: Some(expected_date),
                transaction_hash: None,
                signature_type: String::from("APPROVED_HASH"),
                signature: Some(String::from("0x000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001")),
            },
        )),
        signatures: Some(String::from("0x8b841f4b1b24c4db687bed9d9754253f94cd543d05447b31d32d945be6967a636a454fb210e058f783115c84fd141adf0e8d2fedb17df19419858cbb03fdddb31c000000000000000000000000f2cea96575d6b10f51d9af3b10e3e4e5738aa6bd000000000000000000000000000000000000000000000000000000000000000001")),
    };
    assert!(!multisig_tx.is_erc20_transfer(&None));
    assert!(multisig_tx.is_ether_transfer());
    assert!(!multisig_tx.is_erc721_transfer(&None));
}