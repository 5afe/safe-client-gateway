extern crate reqwest;

use crate::config::{base_transaction_service_url};
use crate::models::backend::about::About;
use anyhow::Result;
use reqwest::Url;

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
