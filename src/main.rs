#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, sqlx};

mod models;
mod repository;
mod api;      
mod views;   
mod cache;

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

// Función para inicializar la conexión a Redis y manejar errores
async fn init_cache(rocket: rocket::Rocket<rocket::Build>) -> Result<rocket::Rocket<rocket::Build>, rocket::Rocket<rocket::Build>> {
    let redis_url = rocket.figment().extract_inner("redis_url")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());
    
    match cache::Cache::new(&redis_url).await {
        Ok(cache) => {
            println!("✅ Cache Redis inicializado correctamente");
            Ok(rocket.manage(cache))
        },
        Err(e) => {
            eprintln!("⚠️  Redis no disponible: {}", e);
            eprintln!("⚠️  Continuando sin cache Redis");
            Ok(rocket) // Continuar sin cache
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        // Inicializar Redis Cache
        .attach(rocket::fairing::AdHoc::on_ignite("Redis Cache", |rocket| async move {
            init_cache(rocket).await.unwrap()
        }))

        // Páginas HTML
        .mount("/", routes![
            views::index::index,
            views::books::books_index,
            views::authors::authors_index,
            views::authors::authors_show,
            views::tables::tables_index,
            views::books_form::new_book_form,
            views::books_form::books_create,
            views::books_form::edit_book_form,
            views::books_form::books_update,
            views::books_show::books_show,
            views::reviews_form::new_review_form,
            views::reviews_form::reviews_create,
            views::reviews_form::edit_review_form,
            views::reviews_form::reviews_update,
            views::books_search::books_search_page,
            views::sales::sales_by_book,

        ])
        // API JSON
        .mount("/api", routes![
            // Authors
            api::authors::get_authors,
            api::authors::get_author_details,
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
            api::books::search_books,

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
