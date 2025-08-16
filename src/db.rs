use rocket_db_pools::sqlx::{self, Row, SqlitePool};
use crate::models::*;

pub async fn init_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Crear tablas si no existen
    sqlx::query(include_str!("../migrations/001_initial_schema.sql"))
        .execute(pool)
        .await?;
    
    // Verificar si ya hay datos
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM authors")
        .fetch_one(pool)
        .await?;
    
    if count == 0 {
        // Poblar con datos de ejemplo
        sqlx::query(include_str!("../migrations/002_sample_data.sql"))
            .execute(pool)
            .await?;
    }
    
    Ok(())
}

// Operaciones para Autores
pub async fn get_all_authors(pool: &SqlitePool) -> Result<Vec<Author>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name, birth_date, country, description FROM authors ORDER BY name")
        .fetch_all(pool)
        .await?;
    
    let authors = rows.iter().map(|row| Author {
        id: Some(row.get("id")),
        name: row.get("name"),
        birth_date: row.get("birth_date"),
        country: row.get("country"),
        description: row.get("description"),
    }).collect();
    
    Ok(authors)
}

pub async fn get_author_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Author>, sqlx::Error> {
    let row = sqlx::query("SELECT id, name, birth_date, country, description FROM authors WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    
    Ok(row.map(|row| Author {
        id: Some(row.get("id")),
        name: row.get("name"),
        birth_date: row.get("birth_date"),
        country: row.get("country"),
        description: row.get("description"),
    }))
}

pub async fn create_author(pool: &SqlitePool, author: &CreateAuthor) -> Result<i32, sqlx::Error> {
    let id = sqlx::query_scalar(
        "INSERT INTO authors (name, birth_date, country, description) VALUES (?, ?, ?, ?) RETURNING id"
    )
    .bind(&author.name)
    .bind(&author.birth_date)
    .bind(&author.country)
    .bind(&author.description)
    .fetch_one(pool)
    .await?;
    
    Ok(id)
}

// Operaciones para Libros
pub async fn get_all_books(pool: &SqlitePool) -> Result<Vec<BookWithAuthor>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT b.id, b.title, b.summary, b.publication_date, b.sales_count, b.author_id,
                a.id as author_id, a.name as author_name, a.birth_date as author_birth_date, 
                a.country as author_country, a.description as author_description
         FROM books b
         JOIN authors a ON b.author_id = a.id
         ORDER BY b.title"
    )
    .fetch_all(pool)
    .await?;
    
    let books = rows.iter().map(|row| BookWithAuthor {
        id: Some(row.get("id")),
        title: row.get("title"),
        summary: row.get("summary"),
        publication_date: row.get("publication_date"),
        sales_count: row.get("sales_count"),
        author: Author {
            id: Some(row.get("author_id")),
            name: row.get("author_name"),
            birth_date: row.get("author_birth_date"),
            country: row.get("author_country"),
            description: row.get("author_description"),
        },
    }).collect();
    
    Ok(books)
}

pub async fn get_book_by_id(pool: &SqlitePool, id: i32) -> Result<Option<BookWithAuthor>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT b.id, b.title, b.summary, b.publication_date, b.sales_count, b.author_id,
                a.id as author_id, a.name as author_name, a.birth_date as author_birth_date, 
                a.country as author_country, a.description as author_description
         FROM books b
         JOIN authors a ON b.author_id = a.id
         WHERE b.id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|row| BookWithAuthor {
        id: Some(row.get("id")),
        title: row.get("title"),
        summary: row.get("summary"),
        publication_date: row.get("publication_date"),
        sales_count: row.get("sales_count"),
        author: Author {
            id: Some(row.get("author_id")),
            name: row.get("author_name"),
            birth_date: row.get("author_birth_date"),
            country: row.get("author_country"),
            description: row.get("author_description"),
        },
    }))
}

pub async fn create_book(pool: &SqlitePool, book: &CreateBook) -> Result<i32, sqlx::Error> {
    let id = sqlx::query_scalar(
        "INSERT INTO books (title, summary, publication_date, author_id) VALUES (?, ?, ?, ?) RETURNING id"
    )
    .bind(&book.title)
    .bind(&book.summary)
    .bind(&book.publication_date)
    .bind(book.author_id)
    .fetch_one(pool)
    .await?;
    
    Ok(id)
}

// Operaciones para Reseñas
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

// Operaciones para Ventas Anuales
pub async fn get_yearly_sales_by_book(pool: &SqlitePool, book_id: i32) -> Result<Vec<YearlySalesWithBook>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT ys.id, ys.book_id, b.title as book_title, ys.year, ys.sales
         FROM yearly_sales ys
         JOIN books b ON ys.book_id = b.id
         WHERE ys.book_id = ?
         ORDER BY ys.year DESC"
    )
    .bind(book_id)
    .fetch_all(pool)
    .await?;
    
    let sales = rows.iter().map(|row| YearlySalesWithBook {
        id: Some(row.get("id")),
        book_id: row.get("book_id"),
        book_title: row.get("book_title"),
        year: row.get("year"),
        sales: row.get("sales"),
    }).collect();
    
    Ok(sales)
}

pub async fn create_yearly_sales(pool: &SqlitePool, sales: &CreateYearlySales) -> Result<i32, sqlx::Error> {
    let id = sqlx::query_scalar(
        "INSERT INTO yearly_sales (book_id, year, sales) VALUES (?, ?, ?) RETURNING id"
    )
    .bind(sales.book_id)
    .bind(sales.year)
    .bind(sales.sales)
    .fetch_one(pool)
    .await?;
    
    Ok(id)
}

// Estadísticas generales
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