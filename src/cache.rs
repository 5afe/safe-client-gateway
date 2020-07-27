use rocket_contrib::databases::redis::{self, Commands};
use serde_json;
use serde::ser::{Serialize};
use anyhow::Result;
use rocket::response::content;

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

impl ServiceCache {
    pub fn fetch(&self, id: &String) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    pub fn create(&self, id: &String, dest: &String, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    pub fn invalidate(&self, id: &String) {
        let _: () = self.del(id).unwrap();
    }

    pub fn cache_resp<R>(&self, key: &String, timeout: usize, resp: impl Fn() -> Result<R>) -> Result<content::Json<String>>
        where R: Serialize
    {
        let cached = self.fetch(key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp = resp()?;
                let resp_string = serde_json::to_string(&resp)?;
                self.create(key, &resp_string, timeout);
                Ok(content::Json(resp_string))
            }
        }
    }

    pub fn request_cached(&self, client: &reqwest::blocking::Client, key: &String, url: &String, timeout: usize) -> Result<String>  {
        let data: String = match self.fetch(&key) {
            Some(cached) => cached,
            None => {
                let response = client.get(url).send()?.text()?;
                self.create(&key, &response, timeout);
                response
            }
        };
        Ok(data)
    }
}