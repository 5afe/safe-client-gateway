use anyhow::Result;
use rocket::response::content;
use rocket_contrib::databases::redis::{self, pipe, Commands, Iter, PipelineCommands};
use serde::ser::Serialize;
use serde_json;

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

impl ServiceCache {
    pub fn fetch(&self, id: &str) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    pub fn create(&self, id: &String, dest: &String, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    pub fn invalidate_pattern(&self, pattern: &String) {
        pipeline_delete(self, self.scan_match(pattern).unwrap());
    }

    pub fn _invalidate(&self, id: &String) {
        let _: () = self.del(id).unwrap();
    }

    pub fn cache_resp<R>(
        &self,
        key: &String,
        timeout: usize,
        resp: impl Fn() -> Result<R>,
    ) -> Result<content::Json<String>>
    where
        R: Serialize,
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

    pub fn request_cached(
        &self,
        client: &reqwest::blocking::Client,
        url: &String,
        timeout: usize,
    ) -> Result<String> {
        let data: String = match self.fetch(&url) {
            Some(cached) => cached,
            None => {
                let response = client.get(url).send()?;
                // Don't cache if it is a Server error
                 if response.status().is_server_error() { anyhow::bail!("Got server error for {}", url); };
                let raw_data = response.text()?;
                self.create(&url, &raw_data, timeout);
                raw_data
            }
        };
        Ok(data)
    }
}

fn pipeline_delete(con: &redis::Connection, keys: Iter<String>) {
    let pipeline = &mut pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.execute(con);
}
