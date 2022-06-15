use bb8_redis::bb8::{self, Pool, PooledConnection};
use bb8_redis::redis::{cmd, AsyncCommands, AsyncIter, Cmd, Pipeline, ToRedisArgs};
use bb8_redis::RedisConnectionManager;

use crate::cache::Cache;
use crate::config::{redis_scan_count, redis_uri, redis_uri_mainnet};

type RedisPool = Pool<RedisConnectionManager>;
type RedisConnection<'a> = PooledConnection<'a, RedisConnectionManager>;

async fn create_pool(for_mainnet: bool) -> RedisPool {
    let target = if for_mainnet {
        redis_uri_mainnet()
    } else {
        redis_uri()
    };
    let manager = RedisConnectionManager::new(target)
        .expect("Establishing connection with redis instance failed");
    bb8::Pool::builder()
        .max_size(15) // default is 10
        .build(manager) // we can technically also set a connection await timeout if necessary
        .await
        .expect("Redis connection pool initialization failed")
}

pub struct ServiceCache(RedisPool);

pub async fn new_service_cache() -> impl Cache {
    ServiceCache(create_pool(false).await)
}

pub async fn new_service_cache_mainnet() -> impl Cache {
    ServiceCache(create_pool(true).await)
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
        let mut con = self.conn().await;
        let keys_cmd = scan_match_count_cmd(pattern, redis_scan_count());
        let mut keys = keys_cmd.iter_async(&mut *con).await.unwrap();
        pipeline_delete(&mut keys)
            .await
            .query_async::<_, ()>(&mut *con)
            .await
            .expect("Pipeline delete error");
    }

    async fn invalidate(&self, id: &str) {
        let _: () = self.conn().await.del(id).await.unwrap();
    }

    async fn info(&self) -> Option<String> {
        let mut conn = self.conn().await;
        cmd("INFO").query_async(&mut *conn).await.ok()
    }
}

async fn pipeline_delete(keys: &mut AsyncIter<'_, String>) -> Pipeline {
    let mut pipeline = Pipeline::new();
    while let Some(key) = keys.next_item().await {
        pipeline.del(key);
    }
    pipeline
}

fn scan_match_count_cmd<P: ToRedisArgs, C: ToRedisArgs>(pattern: P, count: C) -> Cmd {
    let mut cmd = cmd("SCAN");
    cmd.cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count);
    cmd
}
