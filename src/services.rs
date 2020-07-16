pub mod transactions;

fn base_transaction_service_url() -> String {
    let base = dotenv!("TRANSACTION_SERVICE_URL").to_string();
    format!("{}{}", base, "/api/v1")
}
