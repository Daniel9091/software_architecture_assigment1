use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::{Db, repository};
use crate::repository::books as books_repo;
use crate::models::{ApiResponse, BookWithAuthor, CreateBook, UpdateBook};

#[get("/books")]
pub async fn get_books(pool: &State<Db>) -> Json<ApiResponse<Vec<BookWithAuthor>>> {
    match repository::get_all_books(&pool.0).await {
        Ok(books) => Json(ApiResponse::success(books)),
        Err(_) => Json(ApiResponse::<Vec<BookWithAuthor>>::error("Error al obtener libros")),
    }
}

#[get("/books/<id>")]
pub async fn get_book(id: i32, pool: &State<Db>) -> Json<ApiResponse<BookWithAuthor>> {
    match repository::get_book_by_id(&pool.0, id).await {
        Ok(Some(book)) => Json(ApiResponse::success(book)),
        Ok(None) => Json(ApiResponse::<BookWithAuthor>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<BookWithAuthor>::error("Error al obtener libro")),
    }
}

#[post("/books", data = "<book>")]
pub async fn create_book(book: Json<CreateBook>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match repository::create_book(&pool.0, &book).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear libro")),
    }
}

#[put("/books/<id>", data = "<book_update>")]
pub async fn update_book(id: i32, book_update: Json<UpdateBook>, pool: &State<Db>) -> Json<ApiResponse<BookWithAuthor>> {
    match repository::update_book(&pool.0, id, &book_update).await {
        Ok(Some(book)) => Json(ApiResponse::success(book)),
        Ok(None) => Json(ApiResponse::<BookWithAuthor>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<BookWithAuthor>::error("Error al actualizar libro")),
    }
}

#[delete("/books/<id>")]
pub async fn delete_book(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match repository::delete_book(&pool.0, id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::<()>::error("Libro no encontrado")),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar libro")),
    }
}

#[derive(Serialize)]
struct Paged<T> {
    items: Vec<T>,
    total: i64,
    page: i64,
    per_page: i64,
    query: String,
}

#[get("/books/search?<q>&<page>&<per_page>")]
pub async fn search_books(
    q: &str,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: &State<Db>,
) -> Json<ApiResponse<Paged<BookWithAuthor>>> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);

    match books_repo::search_books_by_description(&pool.0, q, page, per_page).await {
        Ok((items, total)) => Json(ApiResponse::success(Paged {
            items,
            total,
            page,
            per_page,
            query: q.to_string(),
        })),
        Err(_) => Json(ApiResponse::error("Error en la búsqueda")),
    }
}