extern crate reqwest;

use crate::config::{base_transaction_service_url};
use anyhow::Result;
use crate::models::service::about::About;

pub fn get_about() -> Result<About> {
    Ok(About {
        transaction_service_base_url: base_transaction_service_url(),
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
