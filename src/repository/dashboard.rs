use rocket_db_pools::sqlx::{self, SqlitePool};
use crate::models::*;

pub async fn get_dashboard_stats(pool: &SqlitePool) -> Result<serde_json::Value, sqlx::Error> {
    let total_authors: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM authors").fetch_one(pool).await?;
    let total_books: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM books").fetch_one(pool).await?;
    let total_reviews: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reviews").fetch_one(pool).await?;
    let total_sales: i64 = sqlx::query_scalar("SELECT SUM(sales) FROM yearly_sales").fetch_one(pool).await?;
    
    let avg_rating: f64 = sqlx::query_scalar("SELECT AVG(rating) FROM reviews").fetch_one(pool).await?;
    
    Ok(serde_json::json!({
        "total_authors": total_authors,
        "total_books": total_books,
        "total_reviews": total_reviews,
        "total_sales": total_sales,
        "average_rating": avg_rating
    }))
} 