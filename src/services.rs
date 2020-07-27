pub mod transactions;

pub fn base_transaction_service_url() -> String {
    format!("{}{}", dotenv!("TRANSACTION_SERVICE_URL"), "/api/v1")
}
