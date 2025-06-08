pub mod key_constants;

use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RedisService {
    client: Arc<Mutex<MultiplexedConnection>>,
}

impl RedisService {
    pub fn new(client: MultiplexedConnection) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    // ====================
    // String 类型操作
    // ====================
    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut client = self.client.lock().await;
        client.set(key, value).await
    }

    pub async fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let mut client = self.client.lock().await;
        client.get(key).await
    }

    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> RedisResult<()> {
        let mut client = self.client.lock().await;
        client.set_ex(key, value, seconds).await
    }

    pub async fn set_nx_ex(&self, key: &str, value: &str, seconds: u64) -> RedisResult<String> {
        let mut client = self.client.lock().await;

        let result: String = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(seconds)
            .arg("NX")
            .query_async(&mut *client)
            .await?;

        Ok(result)
    }

    // pub async fn incr(&mut self, key: &str) -> RedisResult<i64> {
    //     self.client.incr(key, 1).await
    // }

    pub async fn has_key(&self, key: &str) -> RedisResult<bool> {
        let mut client = self.client.lock().await;
        client.exists(key).await
    }

    // ====================
    // Set 类型操作
    // ====================
    // pub async fn s_add(&mut self, key: &str, member: &str) -> RedisResult<()> {
    //     self.client.sadd(key, member).await
    // }
    //
    // pub async fn s_members(&mut self, key: &str) -> RedisResult<Vec<String>> {
    //     self.client.smembers(key).await
    // }
    //
    // pub async fn s_rem(&mut self, key: &str, member: &str) -> RedisResult<()> {
    //     self.client.srem(key, member).await
    // }
}
