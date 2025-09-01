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

    // --- OPERACIONES BÁSICAS QUE FUNCIONAN ---

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

    pub async fn delete(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        conn.del(key).await
    }

    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        conn.exists(key).await
    }
}

// Implementación MÍNIMA para Rocket - SIN ERRORES
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for &'r Cache {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Implementación mínima que funciona
        match request.guard::<&rocket::State<Cache>>().await {
            rocket::request::Outcome::Success(cache) => rocket::request::Outcome::Success(cache),
            _ => rocket::request::Outcome::Forward(rocket::http::Status::InternalServerError),
        }
    }
}