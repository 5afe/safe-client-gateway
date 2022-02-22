use crate::cache::Cache;
use crate::config::{redis_scan_count, redis_uri};
use r2d2::{Pool, PooledConnection};
use redis::{
    self, pipe, AsyncCommands, Cmd, Commands, FromRedisValue, Iter, Pipeline, ToRedisArgs,
};

type RedisPool = Pool<redis::Client>;
type RedisConnection = PooledConnection<redis::Client>;

pub struct ServiceCache(RedisPool);

pub fn create_service_cache() -> ServiceCache {
    ServiceCache(create_pool())
}

fn create_pool() -> RedisPool {
    let client = redis::Client::open(redis_uri()).expect("Starting redis::Client failure");
    Pool::builder()
        .max_size(15)
        .build(client)
        .expect("Redis connection pool build failure")
}

impl ServiceCache {
    fn conn(&self) -> RedisConnection {
        self.0.get().expect("Getting redis connection failed")
    }
}

#[rocket::async_trait]
impl Cache for ServiceCache {
    async fn fetch(&self, id: &str) -> Option<String> {
        match self.conn().get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    async fn create(&self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.conn().pset_ex(id, dest, timeout).unwrap();
    }

    async fn insert_in_hash(&self, hash: &str, id: &str, dest: &str) {
        let _: () = self.conn().hset(hash, id, dest).unwrap();
    }

    async fn get_from_hash(&self, hash: &str, id: &str) -> Option<String> {
        self.conn().hget(hash, id).ok()
    }

    async fn has_key(&self, id: &str) -> bool {
        let result: Option<usize> = self.conn().exists(id).ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    async fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.conn().pexpire(id, timeout).unwrap();
    }

    async fn invalidate_pattern(&self, pattern: &str) {
        let mut con = self.conn();
        let keys_cmd = scan_match_count(pattern, redis_scan_count());
        let keys = keys_cmd.iter(&mut *con).unwrap();
        let delete_cmds = pipeline_delete(keys);
        delete_cmds.execute(&mut *con);
    }

    async fn invalidate(&self, id: &str) {
        let _: () = self.conn().del(id).unwrap();
    }

    async fn info(&self) -> Option<String> {
        let mut con = self.conn();
        redis::cmd("INFO").query(&mut *con).ok()
    }
}

fn pipeline_delete(keys: Iter<String>) -> Pipeline {
    let mut pipeline = pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline
}

fn scan_match_count<P: ToRedisArgs, C: ToRedisArgs>(pattern: P, count: C) -> Cmd {
    let mut cmd = redis::cmd("SCAN");
    cmd.cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count);
    cmd
}
