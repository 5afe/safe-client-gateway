use crate::services::transactions;

#[get("/transactions/<tx_hash>")]
pub fn details(tx_hash: String) -> String {
    transactions::get_transactions_details(tx_hash)
}

#[get("/transactions/about")]
pub fn about() -> String {
    transactions::get_about()
}
