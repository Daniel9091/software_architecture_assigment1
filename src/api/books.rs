use rocket::{serde::json::Json, State};
use serde::Serialize;

use crate::{Db, repository};
use crate::repository::books as books_repo;
use crate::models::{ApiResponse, BookWithAuthor, CreateBook, UpdateBook};
use crate::cache::Cache;


// los voy a deja con los logs para despues poder demostrar que ocupa cahce
#[get("/books")]
pub async fn get_books(
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<Vec<BookWithAuthor>>> {
    println!("🔍 Entrando a get_books");
    // Usar constante desde cache.rs
    let cache_key = Cache::KEY_BOOKS_LIST;
    
    // Intentar obtener del caché primero
    if let Ok(cached_books) = cache.get::<Vec<BookWithAuthor>>(cache_key).await {
        println!("✅ Datos obtenidos del CACHÉ");  // ← Log de caché
        return Json(ApiResponse::success(cached_books));
    }
    println!("🔄 Obteniendo datos de la BASE DE DATOS");  // ← Log de BD
    match repository::get_all_books(&pool.0).await {
        Ok(books) => {
            // Almacenar en caché por 5 minutos usando constante TTL
            let _ = cache.set(cache_key, &books, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Datos guardados en CACHÉ");  // ← Log de guardado
            Json(ApiResponse::success(books))
        },
        Err(_) => Json(ApiResponse::<Vec<BookWithAuthor>>::error("Error al obtener libros")),
    }
}


#[get("/books/<id>")]
pub async fn get_book(
    id: i32, 
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<BookWithAuthor>> {
    println!("🔍 Entrando a get_book para id: {}", id);
    let cache_key = format!("{}{}", Cache::KEY_BOOK_PREFIX, id);
    
    if let Ok(cached_book) = cache.get::<BookWithAuthor>(&cache_key).await {
        println!("✅ Datos del libro {} obtenidos del CACHÉ", id);
        return Json(ApiResponse::success(cached_book));
    }
    println!("🔄 Obteniendo datos del libro {} de la BASE DE DATOS", id);
    match repository::get_book_by_id(&pool.0, id).await {
        Ok(Some(book)) => {
            let _ = cache.set(&cache_key, &book, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Datos del libro {} guardados en CACHÉ", id);
            Json(ApiResponse::success(book))
        },
        Ok(None) => {
            println!("❌ Libro {} no encontrado", id);
            Json(ApiResponse::<BookWithAuthor>::error("Libro no encontrado"))
        },
        Err(_) => {
            println!("❌ Error al obtener libro {}", id);
            Json(ApiResponse::<BookWithAuthor>::error("Error al obtener libro"))
        },
    }
}

// Para los siguietes enpoints no se uso cache por que el tiempo de permanencia en el cache es muy bajo
// Esto evita complejidad innecesaria (hablado con el profesor)

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