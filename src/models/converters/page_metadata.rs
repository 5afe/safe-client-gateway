use crate::models::commons::PageMetadata;
use anyhow::Result;
use itertools::Itertools;
use rocket::http::ext::IntoCollection;
use std::ops::Deref;

impl PageMetadata {
    pub fn to_url_string(&self) -> String {
        return format!("limit={}&offset={}", self.limit, self.offset);
    }

    pub fn from_url_string(encoded: &str) -> Self {
        let mut output = Self {
            offset: 0,
            limit: 20,
        };

        let chunked: Vec<Vec<&str>> = encoded
            .split("&")
            .map(|it| it.split("=").collect())
            .collect();

        chunked.into_iter().for_each(|it| {
            let first = it.first().unwrap_or(&"").deref();
            if first == "limit" {
                output.limit = it.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(0);
            } else if first == "offset" {
                output.offset = it.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(0);
            }
        });

        output
    }
}
