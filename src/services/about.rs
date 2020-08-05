extern crate reqwest;

use crate::config::{base_transaction_service_url};
use crate::models::backend::about::About as AboutDto;
use anyhow::Result;
use reqwest::Url;
use crate::models::service::about::About;
use std::process::Command;

pub fn get_about() -> Result<About> {
    let url_string = format!("{}{}", base_transaction_service_url(), "/about");
    let url = Url::parse(&url_string)?;
    let body = reqwest::blocking::get(url)?.text()?;
    let about_dto: AboutDto = serde_json::from_str(&body)?;

    let command = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg("master")
        .output()?;
    let build_number = std::str::from_utf8(command.stdout.as_slice())?.trim();

    Ok(About {
        transaction_service_base_url: base_transaction_service_url(),
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        transaction_service_version: Some(about_dto.version),
        build_number: Some(build_number.to_string()),
    })
}
