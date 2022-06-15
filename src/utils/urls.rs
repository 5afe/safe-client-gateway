use lazy_static::lazy_static;
use regex::Regex;
use rocket::http::uri::Origin;

use super::context::RequestContext;

lazy_static! {
    static ref IP_ADDRESS: Regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
}

pub fn build_absolute_uri(context: &RequestContext, origin: Origin) -> String {
    format!("{}{}", context.host, origin)
}
