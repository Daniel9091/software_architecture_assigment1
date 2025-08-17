use rocket_db_pools::sqlx::{self, SqlitePool};

pub async fn init_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Asegurar claves foráneas en SQLite
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await?;

    // Crear el esquema (ruta relativa desde src/repository/init.rs)
    sqlx::query(include_str!("../../migrations/001_initial_schema.sql"))
        .execute(pool)
        .await?;

    // ¿Hay autores ya cargados?
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM authors;")
        .fetch_one(pool)
        .await?;

    // Semilla de datos iniciales solo si la tabla está vacía
    if count == 0 {
        sqlx::query(include_str!("../../migrations/002_sample_data.sql"))
            .execute(pool)
            .await?;
    }

    Ok(())
}
