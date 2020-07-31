extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::about::About;
use crate::models::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::Transaction as ServiceTransaction;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::extract_query_string;
use crate::providers::info::InfoProvider;
use reqwest::Url;
use anyhow::Result;

pub fn get_transactions_details(details_id: String) -> String {
    let url_string = format!(
        "{}/transactions/{}",
        base_transaction_service_url(),
        tx_hash
    );
    let url = Url::parse(&url_string).unwrap();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    // let transaction: Box<TransactionDto> = serde_json::from_str(&body).unwrap();
    // let transaction: Transaction = transaction.to_transaction();
    // serde_json::to_string(&transaction).unwrap()
    // let json = serde_json::to_string(&transaction).unwrap();
    body
}