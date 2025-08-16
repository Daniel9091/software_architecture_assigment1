use rocket::{serde::json::Json, State};
use crate::{models::*, db};
use crate::models::ApiResponse;
use crate::Db;

// Rutas para Autores
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

// Rutas para Libros
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

// Rutas para Reseñas
#[get("/books/<book_id>/reviews")]
pub async fn get_book_reviews(book_id: i32, pool: &State<Db>) -> Json<ApiResponse<Vec<ReviewWithBook>>> {
    match db::get_reviews_by_book(&pool.0, book_id).await {
        Ok(reviews) => Json(ApiResponse::success(reviews)),
        Err(_) => Json(ApiResponse::<Vec<ReviewWithBook>>::error("Error al obtener reseñas")),
    }
}

#[post("/reviews", data = "<review>")]
pub async fn create_review(review: Json<CreateReview>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match db::create_review(&pool.0, &review).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear reseña")),
    }
}

// Rutas para Ventas Anuales
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

// Ruta para estadísticas del dashboard
#[get("/dashboard")]
pub async fn get_dashboard_stats(pool: &State<Db>) -> Json<ApiResponse<serde_json::Value>> {
    match db::get_dashboard_stats(&pool.0).await {
        Ok(stats) => Json(ApiResponse::success(stats)),
        Err(_) => Json(ApiResponse::<serde_json::Value>::error("Error al obtener estadísticas")),
    }
} 