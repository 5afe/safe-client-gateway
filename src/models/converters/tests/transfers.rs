mod tests {
    use crate::models::backend::transfers::{EtherTransfer as EtherTransferDto, Erc20Transfer as Erc20TransferDto, Erc721Transfer as Erc721TransferDto, Transfer as TransferDto};
    use crate::models::service::transactions::{TransferInfo, TransactionInfo, TransferDirection, EtherTransfer, Transfer, Erc20Transfer, Erc721Transfer, TransactionStatus};
    use crate::providers::info::*;
    use crate::models::service::transactions::details::TransactionDetails;

    #[test]
    fn erc_20_transfer_dto_to_transaction_info() {
        let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
        let erc_20_transfer = TransferDto::Erc20(serde_json::from_str::<Erc20TransferDto>(crate::json::ERC_20_TRANSFER_WITH_TOKEN_INFO).unwrap());

        let mut mock_info_provider = MockInfoProvider::new();
        mock_info_provider
            .expect_safe_info()
            .times(0);
        mock_info_provider
            .expect_token_info()
            .times(0);

        let expected = TransactionInfo::Transfer(Transfer {
            sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
            recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            direction: TransferDirection::Incoming,
            transfer_info: TransferInfo::Erc20(
                Erc20Transfer {
                    token_address: "0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa".to_string(),
                    value: "1000000000000000000".to_string(),
                    token_name: Some("Dai".to_string()),
                    token_symbol: Some("DAI".to_string()),
                    decimals: Some(18),
                    logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa.png".to_string()),
                }
            ),
        });

        let actual = erc_20_transfer.to_transfer(&mut mock_info_provider, safe_address);

        assert_eq!(expected, actual);
    }

    #[test]
    fn erc_721_transfer_dto_to_transaction_info() {
        let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
        let erc_721_transfer = TransferDto::Erc721(serde_json::from_str::<Erc721TransferDto>(crate::json::ERC_721_TRANSFER_WITH_TOKEN_INFO).unwrap());

        let mut mock_info_provider = MockInfoProvider::new();
        mock_info_provider
            .expect_safe_info()
            .times(0);
        mock_info_provider
            .expect_token_info()
            .times(0);

        let expected = TransactionInfo::Transfer(Transfer {
            sender: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
            recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            direction: TransferDirection::Incoming,
            transfer_info: TransferInfo::Erc721(
                Erc721Transfer {
                    token_address: "0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98".to_string(),
                    token_id: "37".to_string(),
                    token_name: Some("PV Memorial Token".to_string()),
                    token_symbol: Some("PVT".to_string()),
                    logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x8979D84FF2c2B797dFEc02469d3a5322cBEf4b98.png".to_string()),
                }
            ),
        });

        let actual = erc_721_transfer.to_transfer(&mut mock_info_provider, safe_address);

        assert_eq!(expected, actual);
    }

    #[test]
    fn ether_transfer_dto_to_transaction_info() {
        let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
        let ether_transfer_dto = TransferDto::Ether(serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap());

        let mut mock_info_provider = MockInfoProvider::new();
        mock_info_provider
            .expect_safe_info()
            .times(0);
        mock_info_provider
            .expect_token_info()
            .times(0);

        let expected = TransactionInfo::Transfer(Transfer {
            sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
            recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            direction: TransferDirection::Incoming,
            transfer_info: (TransferInfo::Ether(EtherTransfer {
                value: "1000000000000000".to_string(),
            })),
        });

        let actual = ether_transfer_dto.to_transfer(&mut mock_info_provider, safe_address);

        assert_eq!(expected, actual);
    }

    #[test]
    fn unknown_transfer_dto_to_transaction_info() {
        let unknown_transfer_dto = TransferDto::Unknown;
        let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
        let mut mock_info_provider = MockInfoProvider::new();
        mock_info_provider
            .expect_safe_info()
            .times(0);
        mock_info_provider
            .expect_token_info()
            .times(0);

        let actual = unknown_transfer_dto.to_transfer(&mut mock_info_provider, safe_address);

        assert_eq!(TransactionInfo::Unknown, actual);
    }

    #[test]
    fn unknown_transfer_dto_get_execution_time() {
        let unknown_transfer_dto = TransferDto::Unknown;

        let actual = unknown_transfer_dto.get_execution_time();

        assert_eq!(None, actual);
    }

    #[test]
    fn unknown_transfer_dto_get_transaction_hash() {
        let unknown_transfer_dto = TransferDto::Unknown;

        let actual = unknown_transfer_dto.get_transaction_hash();

        assert_eq!(None, actual);
    }

    #[test]
    fn transfer_dto_to_transaction_details() {
        let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
        let ether_transfer_dto = TransferDto::Ether(serde_json::from_str::<EtherTransferDto>(crate::json::ETHER_TRANSFER).unwrap());

        let mut mock_info_provider = MockInfoProvider::new();
        mock_info_provider
            .expect_safe_info()
            .times(0);
        mock_info_provider
            .expect_token_info()
            .times(0);

        let expected = TransactionDetails {
            executed_at: Some(1597733631000),
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Transfer(Transfer {
                sender: "0xfFfa5813ED9a5DB4880D7303DB7d0cBe41bC771F".to_string(),
                recipient: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                direction: TransferDirection::Incoming,
                transfer_info: (TransferInfo::Ether(EtherTransfer {
                    value: "1000000000000000".to_string(),
                })),
            }),
            tx_data: None,
            detailed_execution_info: None,
            tx_hash: Some("0x41b610e8cce50bbe3aa06d6953ecc5f92a838aedc024a265c0afca7ec4f33bdf".to_string()),
        };

        let actual = ether_transfer_dto.to_transaction_details(&mut mock_info_provider, safe_address).unwrap();

        assert_eq!(expected, actual)
    }
}