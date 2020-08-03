#[cfg(test)]
mod test {
    use crate::models::backend::transactions::{Transaction as TransactionDto, ModuleTransaction};
    use crate::providers::info::*;
    use chrono::Utc;
    use crate::models::commons::Operation;
    use crate::models::service::transactions::{Transaction, TransactionStatus, TransactionInfo, Custom};


    #[test]
    fn unknown_tx_to_service_transaction() {
        let unknown_tx = TransactionDto::Unknown;
        let mut mock_info_provider = MockInfoProvider::new();

        let error = unknown_tx.to_service_transaction(&mut mock_info_provider);

        assert!(error.is_err());
    }

    #[test]
    fn module_tx_to_service_transaction() {
        let expected_to = String::from("0x12345789");
        let expected_date = Utc::now();
        let expected_date_in_millis = expected_date.timestamp_millis();
        let module_tx = ModuleTransaction {
            created: None,
            execution_date: expected_date,
            block_number: Some(0),
            transaction_hash: None,
            safe: None,
            module: None,
            to: expected_to.clone(),
            value: None,
            data: None,
            operation: Operation::CALL,
        };

        let actual = ModuleTransaction::to_service_transaction(&module_tx);
        let expected = vec!(Transaction {
            id: String::from("module_<something_else_eventually>"),
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