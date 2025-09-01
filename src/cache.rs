use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::{AsyncCommands, FromRedisValue, RedisError, RedisResult, ToRedisArgs};
use std::time::Duration;

// Alias para el pool de conexiones
pub type RedisPool = Pool<RedisConnectionManager>; 

// Estructura principal del caché
#[derive(Clone)]
pub struct Cache {
    pool: RedisPool,
}

// Implementación de métodos para la estructura Cache
impl Cache {
    // Constantes Utiles
    pub const TTL_5_MIN: Duration = Duration::from_secs(300); // 5 minutos

    //Patrones de claves
    pub const KEY_BOOKS_LIST: &str = "books:list";
    pub const KEY_BOOK_PREFIX: &str = "books:id:";
    pub const KEY_AUTHORS_LIST: &str = "authors:list"; 
    pub const KEY_AUTHOR_PREFIX: &str = "authors:id:";
    pub const KEY_REVIEWS_PREFIX: &str = "reviews:book:";
    pub const KEY_SALES_PREFIX: &str = "sales:book:";

    // Inicialización del caché con la URL de Redis
    // Argumenteos: redis_url: &str - URL de conexión a Redis
    // Retorna: Result<Self, Box<dyn std::error::Error>> - Instancia de Cache o error
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager).await?;
        Ok(Self { pool })
    }

    // --- OPERACIONES BÁSICAS ---

    // Obtener un valor del caché (GET)
    // Argumentos: key: &str - Clave del valor a obtener
    // Retorna: RedisResult<T> - Valor obtenido o error
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> RedisResult<T> {
    let mut conn = self.pool.get().await.map_err(|e| {
        RedisError::from((
            bb8_redis::redis::ErrorKind::IoError,
            "Pool connection error",
            e.to_string(),
        ))
    })?;
    
    // Obtener como string y deserializar desde JSON
    let json_str: Option<String> = conn.get(key).await?;
    match json_str {
        Some(json) => {
            let value: T = serde_json::from_str(&json)
                .map_err(|e| RedisError::from((bb8_redis::redis::ErrorKind::TypeError, "JSON deserialization error", e.to_string())))?;
            Ok(value)
        }
        None => Err(RedisError::from((bb8_redis::redis::ErrorKind::TypeError, "Key not found")))
    }
}



    // Establecer un valor en el caché (SET) con opción de TTL
    // Argumentos:
    // - key: &str - Clave del valor a establecer
    // - value: T - Valor a establecer (debe implementar ToRedisArgs)
    // - ttl: Option<Duration> - Tiempo de vida opcional para el valor (None = sin expiración)
    // Nota: Para consistencia, se recomienda SIEMPRE usar TTL

    // Retorna: RedisResult<()> - Resultado de la operación o error
    
    pub async fn set<T: serde::Serialize + Send + Sync>(
        &self, 
        key: &str, 
        value: &T,  // ← Cambiar a referencia
        ttl: Option<Duration>
    ) -> RedisResult<()> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        
        // Serializar a JSON
        let json_str = serde_json::to_string(value)
            .map_err(|e| RedisError::from((bb8_redis::redis::ErrorKind::TypeError, "JSON serialization error", e.to_string())))?;
        
        if let Some(ttl) = ttl {
            conn.set_ex(key, json_str, ttl.as_secs()).await
        } else {
            conn.set(key, json_str).await
        }
    }


    // Eliminar un valor del caché (DEL)
    // Argumentos: key: &str - Clave del valor a eliminar
    // Retorna: RedisResult<()> - Resultado de la operación o error
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

    // Eliminar múltiples claves que coincidan con un patrón del caché (DEL con patrón)
    // Argumentos: pattern: &str - Patrón para coincidir con las claves a eliminar
    // Retorna: RedisResult<()> - Resultado de la operación o error
    pub async fn delete_pattern(&self, pattern: &str) -> RedisResult<()> {
        let mut conn = self.pool.get().await.map_err(|e| {
            RedisError::from((
                bb8_redis::redis::ErrorKind::IoError,
                "Pool connection error",
                e.to_string(),
            ))
        })?;
        
        // Obtener todas las claves que coinciden con el patrón
        let keys: Vec<String> = conn.keys(pattern).await?;
        
        // Eliminar todas las claves encontradas
        if !keys.is_empty() {
            conn.del(keys).await?;
        }
        
        Ok(())
    }

    // Verificar si una clave existe en el caché (EXISTS)
    // Argumentos: key: &str - Clave a verificar
    // Retorna: RedisResult<bool> - true si existe, false si no, o error
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

// Implementación par inegracion con Rocket
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