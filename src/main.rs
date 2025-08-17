#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, sqlx};

mod models;
mod db;
mod api;      
mod routes;   

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        // Páginas HTML
        .mount("/", routes![
            routes::index::index,
            routes::books::books_index,
            routes::authors::authors_index,
            routes::tables::tables_index,
        ])
        // API JSON
        .mount("/api", routes![
            // Authors
            api::authors::get_authors,
            api::authors::get_author,
            api::authors::create_author,
            api::authors::update_author,
            api::authors::delete_author,

            // Books
            api::books::get_books,
            api::books::get_book,
            api::books::create_book,
            api::books::update_book,
            api::books::delete_book,

            // Reviews
            api::reviews::get_book_reviews,
            api::reviews::create_review,
            api::reviews::update_review,
            api::reviews::delete_review,

            // Sales
            api::sales::get_book_sales,
            api::sales::create_yearly_sales,
            api::sales::update_yearly_sales,
            api::sales::delete_yearly_sales,

            // Dashboard
            api::dashboard::get_dashboard_stats,
        ])

}
