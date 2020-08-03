#[cfg(test)]
mod test {
    use crate::models::backend::transactions::Transaction;
    use crate::providers::info::*;


    #[test]
    fn unknown_tx_to_service_transaction() {
        let unknown_tx = Transaction::Unknown;
        let mut mock_info_provider = MockInfoProvider::new();

        let error = unknown_tx.to_service_transaction(&mut mock_info_provider);

        match error {
            Result::Err(_) => println!("Failed successfully"),
            _ => panic!("Should be error"),
        };
    }
}