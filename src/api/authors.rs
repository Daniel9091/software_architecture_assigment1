use rocket::{serde::json::Json, State};
use crate::{models::*, db};
use crate::models::ApiResponse;
use crate::Db;

#[get("/authors")]
pub async fn get_authors(pool: &State<Db>) -> Json<ApiResponse<Vec<Author>>> {
    match db::get_all_authors(&pool.0).await {
        Ok(authors) => Json(ApiResponse::success(authors)),
        Err(_) => Json(ApiResponse::<Vec<Author>>::error("Error al obtener autores")),
    }
}

#[get("/authors/<id>")]
pub async fn get_author(id: i32, pool: &State<Db>) -> Json<ApiResponse<Author>> {
    match db::get_author_by_id(&pool.0, id).await {
        Ok(Some(author)) => Json(ApiResponse::success(author)),
        Ok(None) => Json(ApiResponse::<Author>::error("Autor no encontrado")),
        Err(_) => Json(ApiResponse::<Author>::error("Error al obtener autor")),
    }
}

#[post("/authors", data = "<author>")]
pub async fn create_author(author: Json<CreateAuthor>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match db::create_author(&pool.0, &author).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear autor")),
    }
}

#[put("/authors/<id>", data = "<author_update>")]
pub async fn update_author(id: i32, author_update: Json<UpdateAuthor>, pool: &State<Db>) -> Json<ApiResponse<Author>> {
    match db::update_author(&pool.0, id, &author_update).await {
        Ok(Some(author)) => Json(ApiResponse::success(author)),
        Ok(None) => Json(ApiResponse::<Author>::error("Autor no encontrado")),
        Err(_) => Json(ApiResponse::<Author>::error("Error al actualizar autor")),
    }
}

#[delete("/authors/<id>")]
pub async fn delete_author(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match db::delete_author(&pool.0, id).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar autor")),
    }
}
