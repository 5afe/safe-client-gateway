#[cfg(test)]
mod test {
    use crate::models::backend::transactions::{Transaction as TransactionDto, ModuleTransaction};
    use crate::providers::info::*;
    use chrono::Utc;
    use crate::models::commons::Operation;
    use crate::models::service::transactions::{TransactionStatus, TransactionInfo, Custom,
                                               ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX,
                                               ID_SEPERATOR};
    use crate::models::service::transactions::summary::TransactionSummary;
    use crate::utils::hex_hash;

    #[test]
    fn unknown_tx_to_service_transaction() {
        let unknown_tx = TransactionDto::Unknown;
        let mut mock_info_provider = MockInfoProvider::new();

        let error = unknown_tx.to_transaction_summary(&mut mock_info_provider, &String::from(""));

        assert!(error.is_err());
    }

    #[test]
    fn module_tx_to_service_transaction() {
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
        let expected: Vec<TransactionSummary> = vec!(
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
    fn ethereum_tx_to_service_transaction() {}
}