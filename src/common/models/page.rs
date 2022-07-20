use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct Page<T> {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub struct PageMetadata {
    pub offset: u64,
    pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SafeList {
    safes: Vec<String>,
}

impl<T> Page<T> {
    pub fn map_inner<U>(self, link_mapper: impl Fn(Option<String>) -> Option<String>) -> Page<U>
    where
        U: From<T>,
    {
        Page {
            next: link_mapper(self.next),
            previous: link_mapper(self.previous),
            results: self.results.into_iter().map(|it| U::from(it)).collect(),
        }
    }
}
