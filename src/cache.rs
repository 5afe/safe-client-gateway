use rocket_contrib::databases::redis::{self, Commands};

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

impl ServiceCache {
    pub fn fetch(&self, id: &String) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            Err(_e) => None,
        }
    }

    pub fn create(&self, id: &String, dest: &String, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    pub fn invalidate(&self, id: &String) {
        let _: () = self.del(id).unwrap();
    }
}

#[macro_export]
macro_rules! cache_resp {
    ($cache: ident, $key:expr, $timeout:expr, $resp:block) => {{
        let key = $key;
        let cached = $cache.fetch(key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp = $resp;
                let resp_string = serde_json::to_string(&resp)?;
                $cache.create(key, &resp_string, $timeout);
                Ok(content::Json(resp_string))
            },
        }
    };
}}