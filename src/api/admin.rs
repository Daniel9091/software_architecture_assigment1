use rocket::{get, post, serde::json::Json, State};
use crate::{Db, models::ApiResponse, search::Search};

#[post("/admin/reindex")]
pub async fn reindex_all(pool: &State<Db>, search: &State<Search>) -> Json<ApiResponse<String>> {
    if !search.enabled() {
        return Json(ApiResponse::error("Search deshabilitado"));
    }

    // Reindex books
    let mut books_count = 0usize;
    if let Ok(books) = crate::repository::get_all_books(&pool.0).await {
        for b in books {
            let _ = search.index_book(&b).await;
            books_count += 1;
        }
    }

    // Reindex reviews
    let mut reviews_count = 0usize;
    if let Ok(revs) = crate::repository::reviews::get_all_reviews(&pool.0).await {
        for r in revs {
            let _ = search.index_review_row(&r).await;
            reviews_count += 1;
        }
    }

    Json(ApiResponse::success(format!("Reindexados {books_count} libros y {reviews_count} reseñas")))
}
