use rocket::{serde::json::Json, State};
use crate::{repository, models::ApiResponse, cache::Cache};
use crate::Db;

#[get("/dashboard")]
pub async fn get_dashboard_stats(
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<serde_json::Value>> {
    println!("🔍 Entrando a get_dashboard_stats");
    let cache_key = Cache::KEY_DASHBOARD_STATS;
    
    if let Ok(cached_stats) = cache.get::<serde_json::Value>(cache_key).await {
        println!("✅ Estadísticas del dashboard obtenidas del CACHÉ");
        return Json(ApiResponse::success(cached_stats));
    }
    println!("🔄 Obteniendo estadísticas del dashboard de la BASE DE DATOS");
    
    match repository::get_dashboard_stats(&pool.0).await {
        Ok(stats) => {
            let _ = cache.set(cache_key, &stats, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Estadísticas del dashboard guardadas en CACHÉ");
            Json(ApiResponse::success(stats))
        },
        Err(_) => Json(ApiResponse::<serde_json::Value>::error("Error al obtener estadísticas")),
    }
}