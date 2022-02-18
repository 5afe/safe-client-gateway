use crate::cache::Cache;
use crate::config::{redis_scan_count, redis_uri};
use bb8_redis::{
    bb8::Pool,
    bb8::{self, PooledConnection},
    redis::{self, cmd, AsyncCommands, FromRedisValue, Iter, Pipeline, ToRedisArgs},
    RedisConnectionManager,
};

type RedisPool = Pool<RedisConnectionManager>;
type RedisConnection<'a> = PooledConnection<'a, RedisConnectionManager>;

async fn create_pool() -> RedisPool {
    let manager = RedisConnectionManager::new(redis_uri())
        .expect("Establishing connection with redis instance failed");
    bb8::Pool::builder()
        .max_size(15) // default is 10
        .build(manager) // we can technically also set a connection await timeout if necessary
        .await
        .expect("Redis connection pool initialization failed")
}

pub struct ServiceCache(RedisPool);

pub async fn create_service_cache() -> ServiceCache {
    ServiceCache(create_pool().await)
}

impl ServiceCache {
    async fn conn(&self) -> RedisConnection<'_> {
        self.0.get().await.unwrap()
    }
}

#[rocket::async_trait]
impl Cache for ServiceCache {
    async fn fetch(&self, id: &str) -> Option<String> {
        match self.conn().await.get(id).await {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    async fn create(&self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.conn().await.pset_ex(id, dest, timeout).await.unwrap();
    }

    async fn insert_in_hash(&self, hash: &str, id: &str, dest: &str) {
        let _: () = self.conn().await.hset(hash, id, dest).await.unwrap();
    }

    async fn get_from_hash(&self, hash: &str, id: &str) -> Option<String> {
        self.conn().await.hget(hash, id).await.ok()
    }

    async fn has_key(&self, id: &str) -> bool {
        let result: Option<usize> = self.conn().await.exists(id).await.ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    async fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.conn().await.pexpire(id, timeout).await.unwrap();
    }

    async fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(
            &mut self.conn().await,
            scan_match_count(&mut self.conn().await, pattern, redis_scan_count()),
        );
    }

    async fn invalidate(&self, id: &str) {
        let _: () = self.conn().await.del(id).await.unwrap();
    }

    async fn info(&self) -> Option<String> {
        let mut conn = self.conn().await;
        cmd("INFO").query_async(&mut *conn).await.ok()
    }
}

fn pipeline_delete(con: &mut RedisConnection, keys: Iter<String>) {
    let pipeline = &mut Pipeline::new();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.query_async(&mut *con);
}

fn scan_match_count<'r, P: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
    con: &'r mut RedisConnection,
    pattern: P,
    count: C,
) -> redis::Iter<'r, RV> {
    let mut cmd = redis::cmd("SCAN");
    cmd.cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count);
    cmd.iter(&mut *con).unwrap()
}
