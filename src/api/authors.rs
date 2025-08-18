use rocket::{serde::json::Json, State};
use rocket::serde::Serialize;
use rocket_db_pools::sqlx::{self, Row};

use crate::{models::*, repository, Db};

#[get("/authors")]
pub async fn get_authors(pool: &State<Db>) -> Json<ApiResponse<Vec<Author>>> {
    match repository::get_all_authors(&pool.0).await {
        Ok(authors) => Json(ApiResponse::success(authors)),
        Err(_) => Json(ApiResponse::<Vec<Author>>::error("Error al obtener autores")),
    }
}

#[get("/authors/<id>")]
pub async fn get_author(id: i32, pool: &State<Db>) -> Json<ApiResponse<Author>> {
    match repository::get_author_by_id(&pool.0, id).await {
        Ok(Some(author)) => Json(ApiResponse::success(author)),
        Ok(None) => Json(ApiResponse::<Author>::error("Autor no encontrado")),
        Err(_) => Json(ApiResponse::<Author>::error("Error al obtener autor")),
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
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BookSummary {
    pub id: i32,
    pub title: String,
    pub publication_date: Option<i32>,
}


/// Respuesta compuesta para el Show de autor
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorDetails {
    pub author: Author,
    pub books: Vec<BookSummary>,
}

/// GET /api/authors/<id>/details
#[get("/authors/<id>/details")]
pub async fn get_author_details(id: i32, pool: &State<Db>) -> Json<ApiResponse<AuthorDetails>> {
    // 1) Autor
    let author_opt = repository::get_author_by_id(&pool.0, id)
        .await
        .ok()
        .flatten();

    let Some(author) = author_opt else {
        return Json(ApiResponse::<AuthorDetails>::error("Autor no encontrado"));
    };

    // 2) Libros del autor (sin macros; usamos `query` + `Row`)
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
    Json(ApiResponse::success(AuthorDetails { author, books }))
}
