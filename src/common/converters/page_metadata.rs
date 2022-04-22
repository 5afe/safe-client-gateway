use std::collections::HashMap;
use std::ops::Deref;

use crate::common::models::page::PageMetadata;

impl PageMetadata {
    pub fn to_url_string(&self) -> String {
        return format!("limit={}&offset={}", self.limit, self.offset);
    }

    pub fn from_cursor(encoded_cursor: &str) -> Self {
        let mut output = Self::default();

        let chunked: Vec<Vec<&str>> = encoded_cursor
            .split("&")
            .map(|it| it.split("=").collect())
            .collect();

        chunked.into_iter().for_each(|it| {
            let first = it.first().unwrap_or(&"").deref();
            if first == "limit" {
                output.limit = it.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(20);
            } else if first == "offset" {
                output.offset = it.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(0);
            }
        });

        output
    }
}

impl From<PageMetadata> for HashMap<String, String> {
    fn from(page_metadata: PageMetadata) -> Self {
        [
            ("limit".to_string(), page_metadata.limit.to_string()),
            ("offset".to_string(), page_metadata.offset.to_string()),
        ]
        .iter()
        .collect()
    }
}

impl Default for PageMetadata {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 20,
        }
    }
}
