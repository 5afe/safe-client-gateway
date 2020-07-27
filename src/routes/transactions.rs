use crate::cache_resp;
use crate::cache::ServiceCache;
use crate::services::transactions;
use rocket::response::content;
use anyhow::Result;

#[get("/transactions/<safe_address>")]
pub fn all(cache: ServiceCache, safe_address: String) -> Result<content::Json<String>> {
    cache.cache_resp(&safe_address, 60 * 2, || { 
        transactions::get_all_transactions(&safe_address)
    })
}

#[get("/transaction/<tx_hash>")]
pub fn details(tx_hash: String) -> content::Json<String> {
    content::Json(transactions::get_transactions_details(tx_hash))
}

#[get("/transactions/about")]
pub fn about(cache: ServiceCache) -> Result<content::Json<String>> {
    cache.cache_resp(&"about_page".to_owned(), 60 * 200, transactions::get_about)
}
