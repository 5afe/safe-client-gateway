use crate::utils::errors::ApiResult;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use rocket::http::uri::Origin;

use super::context::RequestContext;

lazy_static! {
    static ref IP_ADDRESS: Regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
}

pub fn build_manifest_url(url: &str) -> ApiResult<String> {
    let mut url_parts = Url::parse(url).or(Err(api_error!("Not a valid Url")))?;

    if !url_parts.scheme().starts_with("http") {
        Err(api_error!("Invalid scheme"))
    } else if url_parts.host_str().is_none() {
        Err(api_error!("Invalid host"))
    } else if url_parts.host_str() == Some("localhost") {
        Err(api_error!("Localhost not accepted"))
    } else if IP_ADDRESS.captures(url_parts.host_str().unwrap()).is_some() {
        Err(api_error!("IP address not accepted"))
    } else {
        url_parts
            .path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push("manifest.json");
        url_parts.set_query(None);
        Ok(url_parts.to_string())
    }
}

pub fn build_absolute_uri(context: &RequestContext, origin: Origin) -> String {
    format!("{}{}", context.host, origin)
}
