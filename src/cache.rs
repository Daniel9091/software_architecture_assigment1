use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::{AsyncCommands, FromRedisValue, RedisError, RedisResult, ToRedisArgs};
use std::time::Duration;

pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct Cache {
    pool: RedisPool,
}

impl Cache {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager).await?;
        Ok(Self { pool })
    }

    pub async fn get<T: FromRedisValue>(&self, key: &str) -> RedisResult<T> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        conn.get(key).await
    }

    pub async fn set<T: ToRedisArgs + Send + Sync>(
        &self, 
        key: &str, 
        value: T, 
        ttl: Option<Duration>
    ) -> RedisResult<()> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        if let Some(ttl) = ttl {
            conn.set_ex(key, value, ttl.as_secs()).await
        } else {
            conn.set(key, value).await
        }
    }
}