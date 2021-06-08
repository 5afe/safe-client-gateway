mod cache_op_executors;
pub mod cache_operations;
mod inner_cache;
pub mod redis;

#[cfg(test)]
mod tests;

use mockall::automock;

const CACHE_REQS_PREFIX: &'static str = "c_reqs";
const CACHE_RESP_PREFIX: &'static str = "c_resp";
const CACHE_REQS_RESP_PREFIX: &'static str = "c_re";

#[automock]
pub trait Cache: Send + Sync {
    fn fetch(&self, id: &str) -> Option<String>;
    fn create(&self, id: &str, dest: &str, timeout: usize);
    fn insert_in_hash(&self, hash: &str, id: &str, dest: &str);
    fn get_from_hash(&self, hash: &str, id: &str) -> Option<String>;
    fn has_key(&self, id: &str) -> bool;
    fn expire_entity(&self, id: &str, timeout: usize);
    fn invalidate_pattern(&self, pattern: &str);
    fn invalidate(&self, id: &str);
    fn info(&self) -> Option<String>;
}
