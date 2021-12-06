use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    pub fn map_inner<U>(self) -> Page<U>
    where
        U: From<T>,
    {
        Page {
            next: self.next,
            previous: self.previous,
            results: self.results.into_iter().map(|it| U::from(it)).collect(),
        }
    }
}
