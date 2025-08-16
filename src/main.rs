#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, sqlx};
use rocket::fairing::AdHoc;
use rocket::response::Redirect;

mod models;
mod db;
mod routes;
mod web;

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::on_ignite("Initialize Database", |rocket| async move {
            let pool = Db::fetch(&rocket).unwrap();
            if let Err(e) = db::init_database(&pool.0).await {
                eprintln!("Error initializing database: {}", e);
            }
            rocket
        }))
        .mount("/", routes![
            web::index,
            web::books_index,
            web::authors_index,
            web::tables_index
        ])
        .mount("/api", routes![
            routes::get_authors,
            routes::get_author,
            routes::create_author,
            routes::get_books,
            routes::get_book,
            routes::create_book,
            routes::get_book_reviews,
            routes::create_review,
            routes::get_book_sales,
            routes::create_yearly_sales,
            routes::get_dashboard_stats,
        ])
}

#[get("/")]
pub async fn index() -> Redirect {
    // usar ruta literal evita depender de la resolución de uri! 
    Redirect::to("/books")
}


// lanza el servidosr y registra rutas