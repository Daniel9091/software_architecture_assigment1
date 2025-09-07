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

fn escape_like(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '%' | '_' | '\\' => { out.push('\\'); out.push(ch); }
            _ => out.push(ch),
        }
    }
    out
}

/// Construye patrón subsecuencia: "ala" -> "%a%l%a%"
fn subsequence_pattern(q: &str) -> String {
    let mut s = String::from("%");
    for ch in q.chars() {
        s.push(ch);
        s.push('%');
    }
    s
}

pub async fn search_books_by_description(
    pool: &SqlitePool,
    q: &str,
    page: i64,
    per_page: i64,
) -> Result<(Vec<BookWithAuthor>, i64), sqlx::Error> {
    let q = q.trim();
    if q.is_empty() {
        return Ok((Vec::new(), 0));
    }

    // normalización
    let q_norm = q.to_lowercase();
    let q_esc = escape_like(&q_norm);

    // subcadena contigua
    let like_any = format!("%{}%", q_esc);
    // subsecuencia en orden
    let like_subseq = subsequence_pattern(&q_esc);

    // WHERE: cualquier match en título, resumen o autor,
    // ya sea por subcadena contigua O por subsecuencia.
    // Nota: usamos ESCAPE '\' para respetar el escape manual.
    let where_clause = r#"
        (
            LOWER(b.title)   LIKE ? ESCAPE '\'
         OR LOWER(b.summary) LIKE ? ESCAPE '\'
         OR LOWER(a.name)    LIKE ? ESCAPE '\'
         OR LOWER(b.title)   LIKE ? ESCAPE '\'
         OR LOWER(b.summary) LIKE ? ESCAPE '\'
         OR LOWER(a.name)    LIKE ? ESCAPE '\'
        )
    "#;

    // ----- total -----
    let total_sql = format!(
        r#"
        SELECT COUNT(*)
        FROM books b
        JOIN authors a ON b.author_id = a.id
        WHERE {where_clause}
        "#
    );

    let mut tq = sqlx::query_scalar::<_, i64>(&total_sql)
        // contigua
        .bind(&like_any).bind(&like_any).bind(&like_any)
        // subsecuencia
        .bind(&like_subseq).bind(&like_subseq).bind(&like_subseq);

    let total: i64 = tq.fetch_one(pool).await.unwrap_or(0);

    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    // ----- datos paginados -----
    let data_sql = format!(
        r#"
        SELECT
            b.id               AS book_id,
            b.title            AS book_title,
            b.summary          AS book_summary,
            b.publication_date AS book_publication_date,
            b.sales_count      AS book_sales_count,
            a.id               AS author_id,
            a.name             AS author_name,
            a.birth_date       AS author_birth_date,
            a.country          AS author_country,
            a.description      AS author_description
        FROM books b
        JOIN authors a ON b.author_id = a.id
        WHERE {where_clause}
        ORDER BY b.title
        LIMIT ? OFFSET ?
        "#
    );

    let offset = (page.max(1) - 1) * per_page.max(1);

    let rows = sqlx::query(&data_sql)
        // contigua
        .bind(&like_any).bind(&like_any).bind(&like_any)
        // subsecuencia
        .bind(&like_subseq).bind(&like_subseq).bind(&like_subseq)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let items = rows.into_iter().map(|row| BookWithAuthor {
        id: Some(row.get::<i64, _>("book_id") as i32),
        title: row.get::<String, _>("book_title"),
        summary: row.get::<Option<String>, _>("book_summary"),
        publication_date: row.get::<String, _>("book_publication_date"),
        sales_count: (row.get::<i64, _>("book_sales_count") as i32),
        author: Author {
            id: Some(row.get::<i64, _>("author_id") as i32),
            name: row.get::<String, _>("author_name"),
            birth_date: row.get::<String, _>("author_birth_date"),
            country: row.get::<String, _>("author_country"),
            description: row.get::<Option<String>, _>("author_description"),
        },
    }).collect();

    Ok((items, total))
}