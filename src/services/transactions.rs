extern crate reqwest;

use super::base_transaction_service_url;
use crate::models::backend::about::About;
use crate::models::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::Transaction as ServiceTransaction;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::providers::info::InfoProvider;
use reqwest::Url;
use anyhow::Result;

pub fn get_about() -> Result<String> {
    let url_string = format!("{}{}", base_transaction_service_url(), "/about");
    let url = Url::parse(&url_string)?;
    let body = reqwest::blocking::get(url)?.text()?;
    let about: About = serde_json::from_str(&body)?;
    Ok(format!(
        "This is an API wrapper for {}, version {}\nNo guarantees in terms of availability.",
        about.name, about.api_version
    ))
}

pub fn get_transactions_details(tx_hash: String) -> String {
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


pub fn get_all_transactions(context: &Context, safe_address: &String) -> Result<Vec<ServiceTransaction>> {
    let mut info_provider = InfoProvider::new(context);
    info_provider.safe_info(safe_address);
    let url = format!(
        "{}/safes/{}/all-transactions",
        base_transaction_service_url(),
        safe_address
    );
    let body = context.cache().request_cached(&context.client(), &url, 15)?;
    println!("request URL: {}", &url);
    println!("{:#?}", body);
    let transactions: Page<TransactionDto> = serde_json::from_str(&body)?;
    let transactions: Vec<ServiceTransaction> = transactions.results.into_iter()
        .flat_map(|transaction| transaction.to_service_transaction())
        .collect();
    Ok(transactions)
}