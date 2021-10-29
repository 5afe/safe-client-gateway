use crate::cache::Cache;
use crate::config::{redis_scan_count, redis_uri};
use r2d2::{Pool, PooledConnection};
use redis::{self, pipe, Commands, FromRedisValue, Iter, ToRedisArgs};

type RedisPool = Pool<redis::Client>;
type RedisConnection = PooledConnection<redis::Client>;

pub struct ServiceCache(RedisPool);

pub fn create_service_cache() -> ServiceCache {
    ServiceCache(create_pool())
}

fn create_pool() -> RedisPool {
    let client = redis::Client::open(redis_uri()).unwrap();
    Pool::builder().max_size(15).build(client).unwrap()
}

impl ServiceCache {
    fn conn(&self) -> RedisConnection {
        self.0.get().unwrap()
    }
}

impl Cache for ServiceCache {
    fn fetch(&self, id: &str) -> Option<String> {
        match self.conn().get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    fn create(&self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.conn().pset_ex(id, dest, timeout).unwrap();
    }

    fn insert_in_hash(&self, hash: &str, id: &str, dest: &str) {
        let _: () = self.conn().hset(hash, id, dest).unwrap();
    }

    fn get_from_hash(&self, hash: &str, id: &str) -> Option<String> {
        self.conn().hget(hash, id).ok()
    }

    fn has_key(&self, id: &str) -> bool {
        let result: Option<usize> = self.conn().exists(id).ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.conn().pexpire(id, timeout).unwrap();
    }

    fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(
            &mut self.conn(),
            scan_match_count(&mut self.conn(), pattern, redis_scan_count()),
        );
    }

    fn invalidate(&self, id: &str) {
        let _: () = self.conn().del(id).unwrap();
    }

    fn info(&self) -> Option<String> {
        info(&mut self.conn())
    }
}

fn pipeline_delete(con: &mut redis::Connection, keys: Iter<String>) {
    let pipeline = &mut pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.execute(con);
}

fn scan_match_count<'r, P: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
    con: &'r mut redis::Connection,
    pattern: P,
    count: C,
) -> redis::Iter<'r, RV> {
    let mut cmd = redis::cmd("SCAN");
    cmd.cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count);
    cmd.iter(con).unwrap()
}

fn info(con: &mut redis::Connection) -> Option<String> {
    redis::cmd("INFO").query(con).ok()
}
