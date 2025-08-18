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

pub async fn search_books_by_description(
    pool: &SqlitePool,
    q: &str,
    page: i64,
    per_page: i64,
) -> Result<(Vec<BookWithAuthor>, i64), sqlx::Error> {
    let words: Vec<String> = q
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();

    // Si no hay palabras, devolvemos vacío
    if words.is_empty() {
        return Ok((Vec::new(), 0));
    }

    // Construimos filtros OR: (LOWER(b.summary) LIKE ? OR LOWER(b.summary) LIKE ? ...)
    let mut like_clause = String::new();
    for (i, _) in words.iter().enumerate() {
        if i > 0 { like_clause.push_str(" OR "); }
        like_clause.push_str("LOWER(b.summary) LIKE ?");
    }

    // Consulta total
    let total_sql = format!(
        "SELECT COUNT(*)
         FROM books b
         WHERE ({})",
        like_clause
    );

    let mut total_q = sqlx::query_scalar::<_, i64>(&total_sql);
    for w in &words {
        total_q = total_q.bind(format!("%{}%", w));
    }
    let total: i64 = total_q.fetch_one(pool).await.unwrap_or(0);

    // Consulta paginada con join a author
    let data_sql = format!(
        "SELECT b.id, b.title, b.summary, b.publication_date, b.sales_count, b.author_id,
                a.id as author_id, a.name as author_name, a.birth_date as author_birth_date,
                a.country as author_country, a.description as author_description
         FROM books b
         JOIN authors a ON b.author_id = a.id
         WHERE ({})
         ORDER BY b.title
         LIMIT ? OFFSET ?",
        like_clause
    );

    let mut data_q = sqlx::query(&data_sql);
    for w in &words {
        data_q = data_q.bind(format!("%{}%", w));
    }
    let offset = (page.max(1) - 1) * per_page.max(1);
    data_q = data_q.bind(per_page).bind(offset);

    let rows = data_q.fetch_all(pool).await?;

    let items = rows
        .into_iter()
        .map(|row| BookWithAuthor {
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
        })
        .collect();

    Ok((items, total))
}