pub mod transactions;

use std::env;

fn base_transaction_service_url() -> String {
    let transactions_service_host = env::var("TRANSACTION_SERVICE_URL").expect("Must have TRANSACTION_SERVICE_URL with host defined");
    format!("{}{}", transactions_service_host, "/api/v1")
}
