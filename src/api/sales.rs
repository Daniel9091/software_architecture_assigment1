use rocket::{serde::json::Json, State};
use crate::{models::*, db};
use crate::models::ApiResponse;
use crate::Db;

#[get("/books/<book_id>/sales")]
pub async fn get_book_sales(book_id: i32, pool: &State<Db>) -> Json<ApiResponse<Vec<YearlySalesWithBook>>> {
    match db::get_yearly_sales_by_book(&pool.0, book_id).await {
        Ok(sales) => Json(ApiResponse::success(sales)),
        Err(_) => Json(ApiResponse::<Vec<YearlySalesWithBook>>::error("Error al obtener ventas")),
    }
}

#[post("/sales", data = "<sales>")]
pub async fn create_yearly_sales(sales: Json<CreateYearlySales>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match db::create_yearly_sales(&pool.0, &sales).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear registro de ventas")),
    }
}

#[put("/sales/<id>", data = "<sales_update>")]
pub async fn update_yearly_sales(id: i32, sales_update: Json<UpdateYearlySales>, pool: &State<Db>) -> Json<ApiResponse<YearlySalesWithBook>> {
    match db::update_yearly_sales(&pool.0, id, &sales_update).await {
        Ok(Some(sales)) => Json(ApiResponse::success(sales)),
        Ok(None) => Json(ApiResponse::<YearlySalesWithBook>::error("Registro de ventas no encontrado")),
        Err(_) => Json(ApiResponse::<YearlySalesWithBook>::error("Error al actualizar registro de ventas")),
    }
}

#[delete("/sales/<id>")]
pub async fn delete_yearly_sales(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match db::delete_yearly_sales(&pool.0, id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::<()>::error("Registro de ventas no encontrado")),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar registro de ventas")),
    }
}
