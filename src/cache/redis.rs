use crate::cache::Cache;
use crate::config::redis_scan_count;
use r2d2::{Pool, PooledConnection};
use redis::{self, pipe, Commands, FromRedisValue, Iter, ToRedisArgs};
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use std::ops::{Deref, DerefMut};

type RedisPool = Pool<redis::Client>;
type RedisConnection = PooledConnection<redis::Client>;

pub struct ServiceCache(RedisConnection);

impl Deref for ServiceCache {
    type Target = redis::Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ServiceCache {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn create_pool() -> RedisPool {
    // TODO check if we want to use deadpool instead of r2d2
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    Pool::builder().max_size(15).build(client).unwrap()
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'r> for ServiceCache {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = try_outcome!(request.guard::<State<RedisPool>>().await);
        let connection = pool.get().unwrap();
        return request::Outcome::Success(ServiceCache(connection));
    }
}

impl Cache for ServiceCache {
    fn fetch(&mut self, id: &str) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    fn create(&mut self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    fn insert_in_hash(&mut self, hash: &str, id: &str, dest: &str) {
        let _: () = self.hset(hash, id, dest).unwrap();
    }

    fn get_from_hash(&mut self, hash: &str, id: &str) -> Option<String> {
        self.hget(hash, id).ok()
    }

    fn has_key(&mut self, id: &str) -> bool {
        let result: Option<usize> = self.exists(id).ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    fn expire_entity(&mut self, id: &str, timeout: usize) {
        let _: () = self.expire(id, timeout).unwrap();
    }

    fn invalidate_pattern(&mut self, pattern: &str) {
        pipeline_delete(self, scan_match_count(self, pattern, redis_scan_count()));
    }

    fn invalidate(&mut self, id: &str) {
        let _: () = self.del(id).unwrap();
    }

    fn info(&mut self) -> Option<String> {
        info(self)
    }
}

fn pipeline_delete(con: &mut redis::Connection, keys: Iter<String>) {
    let pipeline = &mut pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.execute(con);
}

fn scan_match_count<P: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
    con: &mut redis::Connection,
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

fn info(con: &mut redis::Connection) -> Option<String> {
    redis::cmd("INFO").query(con).ok()
}
