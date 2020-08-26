use crate::models::backend::webhooks::Payload;
use crate::services::hooks::invalidate;
use anyhow::Result;
use crate::utils::cache::Cache;
use reqwest::blocking::Client;
use rocket::response::content::Json;

struct TestCache;

impl Cache for TestCache {
    fn fetch(&self, id: &str) -> Option<String> {
        None
    }

    fn create(&self, id: &String, dest: &String, timeout: usize) {
        println!("unimplemented");
    }

    fn invalidate_pattern(&self, pattern: &String) {
        println!("unimplemented");
    }

    fn _invalidate(&self, id: &String) {
        println!("unimplemented");
    }
}

#[test]
fn some_test() -> Result<()> {
    let payload = Payload {
        address: "".to_string(),
        details: None,
    };


    let cache = TestCache;
    invalidate(&payload, &cache);
    Ok(())
}