use rocket::{serde::json::Json, State};
use crate::{repository, models::ApiResponse};
use crate::Db;

#[get("/dashboard")]
pub async fn get_dashboard_stats(pool: &State<Db>) -> Json<ApiResponse<serde_json::Value>> {
    match repository::get_dashboard_stats(&pool.0).await {
        Ok(stats) => Json(ApiResponse::success(stats)),
        Err(_) => Json(ApiResponse::<serde_json::Value>::error("Error al obtener estadísticas")),
    }
}
