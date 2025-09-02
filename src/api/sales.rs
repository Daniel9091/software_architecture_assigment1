use rocket::{serde::json::Json, State};
use crate::{models::*, repository, cache::Cache};
use crate::models::ApiResponse;
use crate::Db;

#[get("/books/<book_id>/sales")]
pub async fn get_book_sales(
    book_id: i32, 
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<Vec<YearlySalesWithBook>>> {
    println!("🔍 Entrando a get_book_sales para book_id: {}", book_id);
    let cache_key = format!("{}{}", Cache::KEY_SALES_PREFIX, book_id);
    
    if let Ok(cached_sales) = cache.get::<Vec<YearlySalesWithBook>>(&cache_key).await {
        println!("✅ Datos de ventas del libro {} obtenidos del CACHÉ", book_id);
        return Json(ApiResponse::success(cached_sales));
    }
    println!("🔄 Obteniendo datos de ventas del libro {} de la BASE DE DATOS", book_id);
    
    match repository::get_yearly_sales_by_book(&pool.0, book_id).await {
        Ok(sales) => {
            let _ = cache.set(&cache_key, &sales, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Datos de ventas del libro {} guardados en CACHÉ", book_id);
            Json(ApiResponse::success(sales))
        },
        Err(_) => Json(ApiResponse::<Vec<YearlySalesWithBook>>::error("Error al obtener ventas")),
    }
}

#[post("/sales", data = "<sales>")]
pub async fn create_yearly_sales(sales: Json<CreateYearlySales>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match repository::create_yearly_sales(&pool.0, &sales).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear registro de ventas")),
    }
}

#[put("/sales/<id>", data = "<sales_update>")]
pub async fn update_yearly_sales(id: i32, sales_update: Json<UpdateYearlySales>, pool: &State<Db>) -> Json<ApiResponse<YearlySalesWithBook>> {
    match repository::update_yearly_sales(&pool.0, id, &sales_update).await {
        Ok(Some(sales)) => Json(ApiResponse::success(sales)),
        Ok(None) => Json(ApiResponse::<YearlySalesWithBook>::error("Registro de ventas no encontrado")),
        Err(_) => Json(ApiResponse::<YearlySalesWithBook>::error("Error al actualizar registro de ventas")),
    }
}

#[delete("/sales/<id>")]
pub async fn delete_yearly_sales(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match repository::delete_yearly_sales(&pool.0, id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::<()>::error("Registro de ventas no encontrado")),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar registro de ventas")),
    }
}
