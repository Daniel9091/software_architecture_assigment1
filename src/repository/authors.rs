use rocket_db_pools::sqlx::{self, SqlitePool, Row};
use crate::models::*;

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

pub async fn update_author(pool: &SqlitePool, id: i32, author: &UpdateAuthor) -> Result<Option<Author>, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE authors
         SET name = COALESCE(?, name),
             birth_date = COALESCE(?, birth_date),
             country = COALESCE(?, country),
             description = COALESCE(?, description)
         WHERE id = ?
         RETURNING id, name, birth_date, country, description"
    )
    .bind(&author.name)
    .bind(&author.birth_date)
    .bind(&author.country)
    .bind(&author.description)
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

pub async fn delete_author(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    // Primero obtener todos los libros de este autor
    let book_ids: Vec<i32> = sqlx::query("SELECT id FROM books WHERE author_id = ?")
        .bind(id)
        .map(|row: sqlx::sqlite::SqliteRow| row.get("id"))
        .fetch_all(pool)
        .await?;

    // Para cada libro, eliminar reseñas y ventas
    for book_id in book_ids {
        sqlx::query("DELETE FROM reviews WHERE book_id = ?")
            .bind(book_id)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM yearly_sales WHERE book_id = ?")
            .bind(book_id)
            .execute(pool)
            .await?;
    }

    // Luego borrar los libros del autor
    sqlx::query("DELETE FROM books WHERE author_id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    // Finalmente borrar el autor
    sqlx::query("DELETE FROM authors WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
