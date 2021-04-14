mod cache_op_executors;
pub mod cache_operations;
mod inner_cache;
pub mod redis;

#[cfg(test)]
mod tests;

use mockall::automock;

#[automock]
pub trait Cache {
    fn fetch(&mut self, id: &str) -> Option<String>;
    fn create(&mut self, id: &str, dest: &str, timeout: usize);
    fn insert_in_hash(&mut self, hash: &str, id: &str, dest: &str);
    fn get_from_hash(&mut self, hash: &str, id: &str) -> Option<String>;
    fn has_key(&mut self, id: &str) -> bool;
    fn expire_entity(&mut self, id: &str, timeout: usize);
    fn invalidate_pattern(&mut self, pattern: &str);
    fn invalidate(&mut self, id: &str);
    fn info(&mut self) -> Option<String>;
}
