extern crate reqwest;

use super::base_transaction_service_url;
use crate::models::backend::about::About;
use crate::models::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::Transaction;
use crate::models::converters::transactions;
use crate::models::commons::Page;
use reqwest::Url;

pub fn get_about() -> String {
    let url_string = format!("{}{}", base_transaction_service_url(), "/about");
    let url = Url::parse(&url_string).unwrap();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let about: About = serde_json::from_str(&body).unwrap();
    format!(
        "This is an API wrapper for {}, version {}\nNo guarantees in terms of availability.",
        about.name, about.api_version
    )
}

pub fn get_transactions_details(tx_hash: String) -> String {
    let url_string = format!(
        "{}/transactions/{}",
        base_transaction_service_url(),
        tx_hash
    );
    let url = Url::parse(&url_string).unwrap();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let transaction: Box<dyn TransactionDto> = serde_json::from_str(&body).unwrap();
    // let transaction: Transaction = transaction.to_transaction();
    // serde_json::to_string(&transaction).unwrap()
    let json = serde_json::to_string(&transaction).unwrap();
    json
}


pub fn get_all_transactions(safe_address: String) -> String {
    let url_string = format!(
        "{}/safes/{}/all-transactions",
        base_transaction_service_url(),
        safe_address
    );
    let url = Url::parse(&url_string).unwrap();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("request URL: {}", &url_string);
    println!("{:#?}", body);
    let transactions: Page<Box<dyn TransactionDto>> = serde_json::from_str(&body).unwrap();
    let transactions: Vec<Transaction> = transactions.results.into_iter()
        .map(|transaction| transaction.to_service_transaction())//.to_transaction())
        .collect();
    serde_json::to_string(&transactions).unwrap()
    // let json = serde_json::to_string(&transactions).unwrap();
    // json
}