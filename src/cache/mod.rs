use mockall::automock;

mod cache_op_executors;
pub mod cache_operations;
mod inner_cache;
pub mod manager;
pub mod redis;

#[cfg(test)]
mod tests;

const CACHE_REQS_PREFIX: &'static str = "c_reqs";
const CACHE_RESP_PREFIX: &'static str = "c_resp";
const CACHE_REQS_RESP_PREFIX: &'static str = "c_re";

#[automock]
#[rocket::async_trait]
pub trait Cache: Send + Sync {
    async fn fetch(&self, id: &str) -> Option<String>;
    async fn create(&self, id: &str, dest: &str, timeout: usize);
    async fn insert_in_hash(&self, hash: &str, id: &str, dest: &str);
    async fn get_from_hash(&self, hash: &str, id: &str) -> Option<String>;
    async fn has_key(&self, id: &str) -> bool;
    async fn expire_entity(&self, id: &str, timeout: usize);
    async fn invalidate_pattern(&self, pattern: &str);
    async fn invalidate(&self, id: &str);
    async fn info(&self) -> Option<String>;
}
