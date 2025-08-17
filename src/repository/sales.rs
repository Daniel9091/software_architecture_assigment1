use rocket_db_pools::sqlx::{self, SqlitePool, Row};
use crate::models::*;

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

pub async fn update_yearly_sales(pool: &SqlitePool, id: i32, sales: &UpdateYearlySales) -> Result<Option<YearlySalesWithBook>, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE yearly_sales
         SET year = COALESCE(?, year),
             sales = COALESCE(?, sales)
         WHERE id = ?
         RETURNING id, book_id, year, sales"
    )
    .bind(&sales.year)
    .bind(&sales.sales)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| YearlySalesWithBook {
        id: Some(row.get("id")),
        book_id: row.get("book_id"),
        book_title: String::new(), // podés hacer join a books si querés
        year: row.get("year"),
        sales: row.get("sales"),
    }))
}

pub async fn delete_yearly_sales(pool: &SqlitePool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM yearly_sales WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
