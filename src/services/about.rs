extern crate reqwest;

use crate::config::{base_transaction_service_url, build_number, version};
use crate::models::service::about::About;
use crate::utils::errors::ApiResult;

pub async fn get_about() -> ApiResult<About> {
    Ok(About {
        transaction_service_base_url: base_transaction_service_url(),
        name: env!("CARGO_PKG_NAME").to_string(),
        version: version(),
        build_number: build_number(),
    })
}
