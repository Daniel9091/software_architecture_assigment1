use rocket_db_pools::sqlx::{self, SqlitePool, Row};
use crate::models::*;

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

pub async fn update_book(pool: &SqlitePool, id: i32, book: &UpdateBook) -> Result<Option<BookWithAuthor>, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE books
         SET title = COALESCE(?, title),
             summary = COALESCE(?, summary),
             publication_date = COALESCE(?, publication_date),
             author_id = COALESCE(?, author_id)
         WHERE id = ?
         RETURNING id, title, summary, publication_date, sales_count, author_id"
    )
    .bind(&book.title)
    .bind(&book.summary)
    .bind(&book.publication_date)
    .bind(&book.author_id)
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
            name: String::new(),          // ⚠️ Podés hacer join si querés traer el autor completo
            birth_date: String::new(),
            country: String::new(),
            description: None,
        },
    }))
}

pub async fn delete_book(pool: &SqlitePool, id: i32) -> Result<bool, sqlx::Error> {
    // Eliminar primero reseñas
    sqlx::query("DELETE FROM reviews WHERE book_id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    // Eliminar ventas
    sqlx::query("DELETE FROM yearly_sales WHERE book_id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    // Eliminar el libro
    let result = sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
