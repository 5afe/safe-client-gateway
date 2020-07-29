extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration, scheme};
use crate::models::backend::about::About;
use crate::models::backend::transactions::Transaction as TransactionDto;
use crate::models::service::transactions::Transaction as ServiceTransaction;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::providers::info::InfoProvider;
use reqwest::Url;
use anyhow::Result;
use rocket::http::uri::{Absolute, Uri};

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


pub fn get_all_transactions(context: &Context, safe_address: &String, next: &Option<String>, previous: &Option<String>) -> Result<Page<ServiceTransaction>> {
    let mut info_provider = InfoProvider::new(context);
    let url = format!(
        "{}/safes/{}/all-transactions/?{}",
        base_transaction_service_url(),
        safe_address,
        next.as_ref().unwrap_or(&String::new())
    );
    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    println!("request URL: {}", &url);
    println!("next: {:#?}", next);
    println!("{:#?}", body);
    let backend_transactions: Page<TransactionDto> = serde_json::from_str(&body)?;
    let service_transactions: Vec<ServiceTransaction> = backend_transactions.results.into_iter()
        .flat_map(|transaction| transaction.to_service_transaction(&mut info_provider).unwrap_or(vec!()))
        .collect();
    Ok(Page {
        next: backend_transactions.next.as_ref().and_then(|link| {
            Some(format!("{}{}?next={}",
                         context.host().unwrap_or(String::new()),
                         context.origin(),
                         Uri::percent_encode(Absolute::parse(link).ok()?.origin()?.query()?)
            ))
        }),
        previous: backend_transactions.previous.as_ref().and_then(|link| {
            Some(format!("{}{}?next={}",
                         context.host().unwrap_or(String::new()),
                         context.origin(),
                         Uri::percent_encode(Absolute::parse(link).ok()?.origin()?.query()?)
            ))
        }),
        results: service_transactions,
    })
}