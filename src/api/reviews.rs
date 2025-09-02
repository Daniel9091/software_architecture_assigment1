use rocket::{serde::json::Json, State};
use crate::{models::*, repository, cache::Cache};
use crate::models::ApiResponse;
use crate::Db;

#[get("/books/<book_id>/reviews")]
pub async fn get_book_reviews(
    book_id: i32, 
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<Vec<ReviewWithBook>>> {
    println!("🔍 Entrando a get_book_reviews para book_id: {}", book_id);
    let cache_key = format!("{}{}", Cache::KEY_REVIEWS_PREFIX, book_id);
    
    if let Ok(cached_reviews) = cache.get::<Vec<ReviewWithBook>>(&cache_key).await {
        println!("✅ Reseñas del libro {} obtenidas del CACHÉ", book_id);
        return Json(ApiResponse::success(cached_reviews));
    }
    println!("🔄 Obteniendo reseñas del libro {} de la BASE DE DATOS", book_id);
    
    match repository::get_reviews_by_book(&pool.0, book_id).await {
        Ok(reviews) => {
            let _ = cache.set(&cache_key, &reviews, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Reseñas del libro {} guardadas en CACHÉ", book_id);
            Json(ApiResponse::success(reviews))
        },
        Err(_) => Json(ApiResponse::<Vec<ReviewWithBook>>::error("Error al obtener reseñas")),
    }
}

#[post("/reviews", data = "<review>")]
pub async fn create_review(review: Json<CreateReview>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match repository::create_review(&pool.0, &review).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear reseña")),
    }
}

#[put("/reviews/<id>", data = "<review_update>")]
pub async fn update_review(id: i32, review_update: Json<UpdateReview>, pool: &State<Db>) -> Json<ApiResponse<ReviewWithBook>> {
    match repository::update_review(&pool.0, id, &review_update).await {
        Ok(Some(review)) => Json(ApiResponse::success(review)),
        Ok(None) => Json(ApiResponse::<ReviewWithBook>::error("Reseña no encontrada")),
        Err(_) => Json(ApiResponse::<ReviewWithBook>::error("Error al actualizar reseña")),
    }
}

#[delete("/reviews/<id>")]
pub async fn delete_review(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match repository::delete_review(&pool.0, id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::<()>::error("Reseña no encontrada")),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar reseña")),
    }
}
