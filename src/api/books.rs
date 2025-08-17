use rocket::{serde::json::Json, State};
use crate::{models::*, db};
use crate::models::ApiResponse;
use crate::Db;

#[get("/books")]
pub async fn get_books(pool: &State<Db>) -> Json<ApiResponse<Vec<BookWithAuthor>>> {
    match db::get_all_books(&pool.0).await {
        Ok(books) => Json(ApiResponse::success(books)),
        Err(_) => Json(ApiResponse::<Vec<BookWithAuthor>>::error("Error al obtener libros")),
    }
}

#[get("/books/<id>")]
pub async fn get_book(id: i32, pool: &State<Db>) -> Json<ApiResponse<BookWithAuthor>> {
    match db::get_book_by_id(&pool.0, id).await {
        Ok(Some(book)) => Json(ApiResponse::success(book)),
        Ok(None) => Json(ApiResponse::<BookWithAuthor>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<BookWithAuthor>::error("Error al obtener libro")),
    }
}

#[post("/books", data = "<book>")]
pub async fn create_book(book: Json<CreateBook>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match db::create_book(&pool.0, &book).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear libro")),
    }
}

#[put("/books/<id>", data = "<book_update>")]
pub async fn update_book(id: i32, book_update: Json<UpdateBook>, pool: &State<Db>) -> Json<ApiResponse<BookWithAuthor>> {
    match db::update_book(&pool.0, id, &book_update).await {
        Ok(Some(book)) => Json(ApiResponse::success(book)),
        Ok(None) => Json(ApiResponse::<BookWithAuthor>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<BookWithAuthor>::error("Error al actualizar libro")),
    }
}

#[delete("/books/<id>")]
pub async fn delete_book(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match db::delete_book(&pool.0, id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::<()>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar libro")),
    }
}
