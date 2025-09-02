use rocket::{serde::json::Json, State};
use rocket::serde::{Serialize, Deserialize}; 
use rocket_db_pools::sqlx::{self, Row};

use crate::{models::*, repository, Db, cache::Cache};

#[get("/authors")]
pub async fn get_authors(
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<Vec<Author>>> {
    println!("🔍 Entrando a get_authors");
    let cache_key = Cache::KEY_AUTHORS_LIST;
    
    if let Ok(cached_authors) = cache.get::<Vec<Author>>(cache_key).await {
        println!("✅ Datos de autores obtenidos del CACHÉ");
        return Json(ApiResponse::success(cached_authors));
    }
    println!("🔄 Obteniendo datos de autores de la BASE DE DATOS");
    match repository::get_all_authors(&pool.0).await {
        Ok(authors) => {
            let _ = cache.set(cache_key, &authors, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Datos de autores guardados en CACHÉ");
            Json(ApiResponse::success(authors))
        },
        Err(_) => Json(ApiResponse::<Vec<Author>>::error("Error al obtener autores")),
    }
}


// No se sabe donde se ocupa este endpoit
// tiene implementado el cache igual
#[get("/authors/<id>")]
pub async fn get_author(
    id: i32, 
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<Author>> {
    println!("🔍 Entrando a get_author para id: {}", id);
    let cache_key = format!("{}{}", Cache::KEY_AUTHOR_PREFIX, id);
    
    if let Ok(cached_author) = cache.get::<Author>(&cache_key).await {
        println!("✅ Datos del autor {} obtenidos del CACHÉ", id);
        return Json(ApiResponse::success(cached_author));
    }
    println!("🔄 Obteniendo datos del autor {} de la BASE DE DATOS", id);
    match repository::get_author_by_id(&pool.0, id).await {
        Ok(Some(author)) => {
            let _ = cache.set(&cache_key, &author, Some(Cache::TTL_5_MIN)).await;
            println!("💾 Datos del autor {} guardados en CACHÉ", id);
            Json(ApiResponse::success(author))
        },
        Ok(None) => {
            println!("❌ Autor {} no encontrado", id);
            Json(ApiResponse::<Author>::error("Autor no encontrado"))
        },
        Err(_) => {
            println!("❌ Error al obtener autor {}", id);
            Json(ApiResponse::<Author>::error("Error al obtener autor"))
        },
    }
}


#[post("/authors", data = "<author>")]
pub async fn create_author(author: Json<CreateAuthor>, pool: &State<Db>) -> Json<ApiResponse<i32>> {
    match repository::create_author(&pool.0, &author).await {
        Ok(id) => Json(ApiResponse::success(id)),
        Err(_) => Json(ApiResponse::<i32>::error("Error al crear autor")),
    }
}

#[put("/authors/<id>", data = "<author_update>")]
pub async fn update_author(id: i32, author_update: Json<UpdateAuthor>, pool: &State<Db>) -> Json<ApiResponse<Author>> {
    match repository::update_author(&pool.0, id, &author_update).await {
        Ok(Some(author)) => Json(ApiResponse::success(author)),
        Ok(None) => Json(ApiResponse::<Author>::error("Autor no encontrado")),
        Err(_) => Json(ApiResponse::<Author>::error("Error al actualizar autor")),
    }
}

#[delete("/authors/<id>")]
pub async fn delete_author(id: i32, pool: &State<Db>) -> Json<ApiResponse<()>> {
    match repository::delete_author(&pool.0, id).await {
        Ok(_) => Json(ApiResponse::success(())),
        Err(_) => Json(ApiResponse::<()>::error("Error al eliminar autor")),
    }
}

/// DTO liviano para la lista de libros del autor
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BookSummary {
    pub id: i32,
    pub title: String,
    pub publication_date: Option<i32>,
}

/// Respuesta compuesta para el Show de autor
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorDetails {
    pub author: Author,
    pub books: Vec<BookSummary>,
}


// GET /api/authors/<id>/details
// Este enpoint se lleva toda la carga de obtener la informacion del autor y sus libros
#[get("/authors/<id>/details")]
pub async fn get_author_details(
    id: i32, 
    pool: &State<Db>,
    cache: &Cache
) -> Json<ApiResponse<AuthorDetails>> {
    println!("🔍 Entrando a get_author_details para id: {}", id);
    let cache_key = format!("{}{}", Cache::KEY_AUTHOR_DETAILS_PREFIX, id);
    
    if let Ok(cached_details) = cache.get::<AuthorDetails>(&cache_key).await {
        println!("✅ Detalles del autor {} obtenidos del CACHÉ", id);
        return Json(ApiResponse::success(cached_details));
    }
    println!("🔄 Obteniendo detalles del autor {} de la BASE DE DATOS", id);
    
    // 1) Autor
    let author_opt = repository::get_author_by_id(&pool.0, id)
        .await
        .ok()
        .flatten();

    let Some(author) = author_opt else {
        println!("❌ Autor {} no encontrado", id);
        return Json(ApiResponse::<AuthorDetails>::error("Autor no encontrado"));
    };

    // 2) Libros del autor
    let rows = sqlx::query(
        r#"
        SELECT 
        id, 
        title, 
        CAST(strftime('%Y', publication_date) AS INTEGER) AS publication_date
        FROM books
        WHERE author_id = ?
        ORDER BY publication_date DESC, title ASC
        "#,
    )
    .bind(id)
    .fetch_all(&pool.0)
    .await
    .unwrap_or_default();

    let books = rows
        .into_iter()
        .map(|r| BookSummary {
            id: r.get("id"),
            title: r.get("title"),
            publication_date: r.get("publication_date"),
        })
        .collect();
    
    let author_details = AuthorDetails { author, books };
    let _ = cache.set(&cache_key, &author_details, Some(Cache::TTL_5_MIN)).await;
    println!("💾 Detalles del autor {} guardados en CACHÉ", id);
    
    Json(ApiResponse::success(author_details))
}