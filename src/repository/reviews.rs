use rocket_db_pools::sqlx::{self, SqlitePool, Row};
use crate::models::*;

pub async fn get_reviews_by_book(pool: &SqlitePool, book_id: i32) -> Result<Vec<ReviewWithBook>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT r.id, r.book_id, b.title as book_title, r.review_text, r.rating, r.positive_votes, r.created_at
         FROM reviews r
         JOIN books b ON r.book_id = b.id
         WHERE r.book_id = ?
         ORDER BY r.created_at DESC"
    )
    .bind(book_id)
    .fetch_all(pool)
    .await?;
    
    let reviews = rows.iter().map(|row| ReviewWithBook {
        id: Some(row.get("id")),
        book_id: row.get("book_id"),
        book_title: row.get("book_title"),
        review_text: row.get("review_text"),
        rating: row.get("rating"),
        positive_votes: row.get("positive_votes"),
        created_at: row.get("created_at"),
    }).collect();
    
    Ok(reviews)
}

pub async fn create_review(pool: &SqlitePool, review: &CreateReview) -> Result<i32, sqlx::Error> {
    let id = sqlx::query_scalar(
        "INSERT INTO reviews (book_id, review_text, rating) VALUES (?, ?, ?) RETURNING id"
    )
    .bind(review.book_id)
    .bind(&review.review_text)
    .bind(review.rating)
    .fetch_one(pool)
    .await?;
    
    Ok(id)
}

pub async fn update_review(pool: &SqlitePool, id: i32, review: &UpdateReview) -> Result<Option<ReviewWithBook>, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE reviews
         SET review_text = COALESCE(?, review_text),
             rating = COALESCE(?, rating),
             positive_votes = COALESCE(?, positive_votes)
         WHERE id = ?
         RETURNING id, book_id, review_text, rating, positive_votes, created_at"
    )
    .bind(&review.review_text)
    .bind(&review.rating)
    .bind(&review.positive_votes)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| ReviewWithBook {
        id: Some(row.get("id")),
        book_id: row.get("book_id"),
        book_title: String::new(), // si querés traete el título con un JOIN
        review_text: row.get("review_text"),
        rating: row.get("rating"),
        positive_votes: row.get("positive_votes"),
        created_at: row.get("created_at"),
    }))
}

pub async fn delete_review(pool: &SqlitePool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM reviews WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
