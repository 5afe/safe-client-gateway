use crate::cache::Cache;
use crate::config::redis_scan_count;
use rocket_contrib::databases::redis::{
    self, pipe, Commands, FromRedisValue, Iter, PipelineCommands, ToRedisArgs,
};

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

impl Cache for ServiceCache {
    fn fetch(&self, id: &str) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    fn create(&self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    fn insert_in_hash(&self, hash: &str, id: &str, dest: &str) {
        let _: () = self.hset(hash, id, dest).unwrap();
    }

    fn get_from_hash(&self, hash: &str, id: &str) -> Option<String> {
        self.hget(hash, id).ok()
    }

    fn has_key(&self, id: &str) -> bool {
        let result: Option<usize> = self.exists(id).ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.expire(id, timeout).unwrap();
    }

    fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(self, scan_match_count(self, pattern, redis_scan_count()));
    }

    fn invalidate(&self, id: &str) {
        let _: () = self.del(id).unwrap();
    }

    fn info(&self) -> Option<String> {
        info(self)
    }
}

fn pipeline_delete(con: &redis::Connection, keys: Iter<String>) {
    let pipeline = &mut pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.execute(con);
}

fn scan_match_count<P: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
    con: &redis::Connection,
    pattern: P,
    count: C,
) -> redis::Iter<RV> {
    redis::cmd("SCAN")
        .cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count)
        .iter(con)
        .unwrap()
}

fn info(con: &redis::Connection) -> Option<String> {
    redis::cmd("INFO").query(con).ok()
}
