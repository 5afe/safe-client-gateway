use crate::models::converters::transactions::{data_size};
use crate::models::backend::transactions::{Transaction as TransactionDto, ModuleTransaction, EthereumTransaction, CreationTransaction, MultisigTransaction};
use crate::providers::info::*;
use chrono::Utc;
use crate::models::commons::Operation;
use crate::models::service::transactions::{TransactionStatus, TransactionInfo, Custom, ID_PREFIX_MULTISIG_TX, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_CREATION_TX, ID_PREFIX_MODULE_TX, Transfer, TransferDirection, TransferInfo, EtherTransfer, Creation, Erc20Transfer};
use crate::models::service::transactions::summary::{TransactionSummary, ExecutionInfo};
use crate::utils::hex_hash;
use crate::models::backend::transfers::{EtherTransfer as EtherTransferDto, Transfer as TransferDto};

#[test]
fn data_size_calculation() {
    assert_eq!(data_size(&None), "0");
    assert_eq!(data_size(&Some(String::from(""))), "0");
    assert_eq!(data_size(&Some(String::from("0x"))), "0");
    assert_eq!(
        data_size(&Some(String::from("0x8d80ff0a000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000f2001230b3d59858296a31053c1b8562ecf89a2f888b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000247de7edef00000000000000000000000034cfac646f301356faa8b21e94227e3583fe3f5f001230b3d59858296a31053c1b8562ecf89a2f888b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024f08a0323000000000000000000000000d5d82b6addc9027b22dca772aa68d5d74cdbdf440000000000000000000000000000"))),
        "324"
    );
}

#[test]
fn unknown_tx_to_summary_transaction() {
    let unknown_tx = TransactionDto::Unknown;
    let mut mock_info_provider = MockInfoProvider::new();

    let error = unknown_tx.to_transaction_summary(&mut mock_info_provider, &String::from(""));

    assert!(error.is_err());
}

#[test]
fn module_tx_to_summary_transaction() {
    let expected_to = String::from("0x12345789");
    let expected_date = Utc::now();
    let expected_date_in_millis = expected_date.timestamp_millis();
    let module_tx = ModuleTransaction {
        created: String::from("created"),
        execution_date: expected_date,
        block_number: 0,
        transaction_hash: String::from("tx_hash"),
        safe: String::from("safe"),
        module: String::from("module"),
        to: expected_to.clone(),
        value: None,
        data: None,
        data_decoded: None,
        operation: Operation::CALL,
    };

    let actual = ModuleTransaction::to_transaction_summary(&module_tx);
    let expected = vec!(
        TransactionSummary {
            id: create_id!(
                ID_PREFIX_MODULE_TX,
                module_tx.safe,
                module_tx.block_number,
                hex_hash(&module_tx)
            ),
            timestamp: expected_date_in_millis,
            tx_status: TransactionStatus::Success,
            execution_info: None,
            tx_info: TransactionInfo::Custom(
                Custom {
                    to: expected_to,
                    data_size: String::from("0"),
                    value: String::from("0"),
                }),
        });
    assert_eq!(actual, expected);
}

#[test]
fn ethereum_tx_to_summary_transaction_no_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();

    let ethereum_tx = EthereumTransaction {
        execution_date: Utc::now(),
        to: String::from("0x1234"),
        data: None,
        tx_hash: String::from("0x4321"),
        block_number: 0,
        transfers: None,
        from: String::from("0x6789"),
    };

    let actual = EthereumTransaction::to_transaction_summary(&ethereum_tx, &mut mock_info_provider, &safe_address);
    assert_eq!(actual, Vec::new());
}

#[test]
fn ethereum_tx_to_summary_transaction_with_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();
    let timestamp = Utc::now();
    let timestamp_millis = timestamp.timestamp_millis();

    let transfers = vec!(
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        }),
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        })
    );
    let ethereum_tx = EthereumTransaction {
        execution_date: timestamp,
        to: String::from("0x1234"),
        data: None,
        tx_hash: String::from("0x4321"),
        block_number: 0,
        transfers: Some(transfers.to_vec()),
        from: String::from("0x6789"),
    };

    let actual = EthereumTransaction::to_transaction_summary(&ethereum_tx, &mut mock_info_provider, &safe_address);
    let expected = vec!(
        TransactionSummary {
            id: create_id!(
                    ID_PREFIX_ETHEREUM_TX,
                    safe_address,
                    ethereum_tx.block_number,
                    hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(0).unwrap())
                ),
            timestamp: timestamp_millis,
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Transfer(Transfer {
                sender: "".to_string(),
                recipient: "".to_string(),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::Ether(
                    EtherTransfer {
                        value: "1".to_string(),
                    }),
            }),
            execution_info: None,
        }, TransactionSummary {
            id: create_id!(
                    ID_PREFIX_ETHEREUM_TX,
                    safe_address,
                    ethereum_tx.block_number,
                    hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(1).unwrap())
                ),
            timestamp: timestamp_millis,
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Transfer(Transfer {
                sender: "".to_string(),
                recipient: "".to_string(),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::Ether(
                    EtherTransfer {
                        value: "1".to_string(),
                    }),
            }),
            execution_info: None,
        });
    assert_eq!(actual, expected);
}

#[test]
fn creation_transaction_to_summary() {
    let created_date = Utc::now();
    let safe_address = String::from("0x38497");
    let creator = String::from("0x123");
    let transaction_hash = String::from("0x2232");
    let factory_address = String::from("0x123");
    let master_copy = String::from("0x987");
    let creation_tx = CreationTransaction {
        created: created_date,
        creator: creator.clone(),
        transaction_hash: transaction_hash.clone(),
        factory_address: Some(factory_address.clone()),
        master_copy: Some(master_copy.clone()),
        setup_data: None,
        data_decoded: None,
    };
    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_CREATION_TX, safe_address),
        timestamp: created_date.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Creation(
            Creation {
                creator: creator,
                transaction_hash: transaction_hash,
                master_copy: Some(master_copy),
                factory: Some(factory_address),
            }
        ),
        execution_info: None,
    };

    let actual = creation_tx.to_transaction_summary(&safe_address);

    assert_eq!(expected, actual);
}

#[test]
fn multisig_transaction_to_erc20_transfer_summary() {
    let safe_info_json = r#"{
      "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
      "nonce": 180,
      "threshold": 3,
      "owners": [
        "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23",
        "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72",
        "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D",
        "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd",
        "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
      ],
      "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
      "modules": [
        "0x25F73b24B866963B0e560fFF9bbA7908be0263E8",
        "0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c",
        "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d"
      ],
      "fallbackHandler": "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44",
      "version": "1.1.1"
    }"#;

    let multisend_tx_json = r#"{
      "safe": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
      "to": "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02",
      "value": "0",
      "data": "0xa9059cbb00000000000000000000000065f8236309e5a99ff0d129d04e486ebce20dc7b000000000000000000000000000000000000000000000000000002d79883d2000",
      "operation": 0,
      "gasToken": "0x0000000000000000000000000000000000000000",
      "safeTxGas": 35601,
      "baseGas": 0,
      "gasPrice": "0",
      "refundReceiver": "0x0000000000000000000000000000000000000000",
      "nonce": 178,
      "executionDate": "2020-08-07T09:06:14Z",
      "submissionDate": "2020-08-07T08:51:35.193667Z",
      "modified": "2020-08-07T08:51:35.193667Z",
      "blockNumber": 6975704,
      "transactionHash": "0x08cf4bb6fe2a7e77e86e7679ec3c266516155cdc9900cdd47afef791169d6e21",
      "safeTxHash": "0x95e32bb8cb88ecdc45732c0a551eae7b3744187cf1ba19cda1440eaaf7b4950c",
      "executor": "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23",
      "isExecuted": true,
      "isSuccessful": true,
      "ethGasPrice": "1000000000",
      "gasUsed": 68346,
      "fee": "68346000000000",
      "origin": null,
      "dataDecoded": {
        "method": "transfer",
        "parameters": [
          {
            "name": "to",
            "type": "address",
            "value": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
          },
          {
            "name": "value",
            "type": "uint256",
            "value": "50000000000000"
          }
        ]
      },
      "confirmationsRequired": 3,
      "confirmations": [
        {
          "owner": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
          "submissionDate": "2020-08-07T08:51:35.213346Z",
          "transactionHash": null,
          "confirmationType": "CONFIRMATION",
          "signature": "0x2e8da6ff14c68a91caecbfdf71cdd134673f33903404bbaa2624808d7f728d0f551d26342f6eb8afa44cab779921b559311b03b7818088cbe8ca2861c7f473f81b",
          "signatureType": "EOA"
        },
        {
          "owner": "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd",
          "submissionDate": "2020-08-07T09:04:34.526493Z",
          "transactionHash": null,
          "confirmationType": "CONFIRMATION",
          "signature": "0x1018509bc2fc5f759430fa5740eeb49fe3699932ecf6539f56d42990a4afc782767961c9c88b8f7b5bba6b7b1637128b4b32c82f481f944dc7880787ce2d43b71b",
          "signatureType": "EOA"
        },
        {
          "owner": "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23",
          "submissionDate": "2020-08-07T09:05:06.133804Z",
          "transactionHash": null,
          "confirmationType": "CONFIRMATION",
          "signature": "0x000000000000000000000000bea2f9227230976d2813a2f8b922c22be1de1b23000000000000000000000000000000000000000000000000000000000000000001",
          "signatureType": "APPROVED_HASH"
        }
      ],
      "signatures": "0x2e8da6ff14c68a91caecbfdf71cdd134673f33903404bbaa2624808d7f728d0f551d26342f6eb8afa44cab779921b559311b03b7818088cbe8ca2861c7f473f81b000000000000000000000000bea2f9227230976d2813a2f8b922c22be1de1b230000000000000000000000000000000000000000000000000000000000000000011018509bc2fc5f759430fa5740eeb49fe3699932ecf6539f56d42990a4afc782767961c9c88b8f7b5bba6b7b1637128b4b32c82f481f944dc7880787ce2d43b71b",
      "transfers": [
        {
          "type": "ERC20_TRANSFER",
          "executionDate": "2020-08-07T09:06:14Z",
          "blockNumber": 6975704,
          "transactionHash": "0x08cf4bb6fe2a7e77e86e7679ec3c266516155cdc9900cdd47afef791169d6e21",
          "to": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
          "value": "50000000000000",
          "tokenId": null,
          "tokenAddress": "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02",
          "tokenInfo": {
            "type": "ERC20",
            "address": "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02",
            "name": "Compound USDT",
            "symbol": "USDT",
            "decimals": 18,
            "logoUri": "https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png"
          },
          "from": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b"
        }
      ],
      "txType": "MULTISIG_TRANSACTION"
    }"#;
    let token_info = TokenInfo {
        token_type: TokenType::Erc20,
        address: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
        decimals: 18,
        symbol: "USDT".to_string(),
        name: "Compound USDT".to_string(),
        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png".to_string()),
    };
    let multisig_tx = serde_json::from_str::<MultisigTransaction>(multisend_tx_json).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(safe_info_json).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_MULTISIG_TX, "0x95e32bb8cb88ecdc45732c0a551eae7b3744187cf1ba19cda1440eaaf7b4950c"),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            recipient: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Erc20(Erc20Transfer {
                token_address: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
                token_name: Some("Compound USDT".to_string()),
                token_symbol: Some("USDT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png".to_string()),
                decimals: Some(18),
                value: "50000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo {
            nonce: 178,
            confirmations_required: 3,
            confirmations_submitted: 3,
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}