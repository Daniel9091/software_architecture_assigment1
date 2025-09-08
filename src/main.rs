#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket_db_pools::{Database, sqlx};

mod models;
mod repository;
mod api;
mod views;
mod cache;
mod opensearch_client;
mod routes_suggest;
mod routes_search;

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);


#[get("/health")]
fn health() -> &'static str { "ok" }

// Inicializa Redis (no rompe si no está disponible)
async fn init_cache(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let redis_url = rocket.figment()
        .extract_inner::<String>("redis_url")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());

    match cache::Cache::new(&redis_url).await {
        Ok(cache) => {
            println!("✅ Cache Redis inicializado correctamente");
            Ok(rocket.manage(cache))
        }
        Err(e) => {
            eprintln!("⚠️  Redis no disponible: {e}");
            eprintln!("⚠️  Continuando sin cache Redis");
            Ok(rocket) // continuar sin cache
        }
    }
}

// Construye Rocket montando estáticos sólo si SERVE_STATIC=true
fn build_rocket() -> Rocket<Build> {
    let mut app = rocket::build()
        .attach(Db::init())
        .attach(rocket::fairing::AdHoc::on_ignite("Redis Cache", |rocket| async move {
            init_cache(rocket).await.unwrap()
        }))
        // Vistas (HTML)
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
            health,
            routes_suggest::suggest,
            routes_search::search
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
        ]);

    // Solo servir archivos estáticos si SERVE_STATIC=true (modo sin proxy)
    let serve_static = std::env::var("SERVE_STATIC").unwrap_or_default() == "true";
    if serve_static {
        // /static → archivos “fijos” (css/js/img)
        app = app.mount("/static", FileServer::from("public/static"));
        // /uploads → imágenes subidas (autores/portadas)
        app = app.mount("/uploads", FileServer::from("public/uploads"));
        println!("📁 Sirviendo estáticos desde public/ (SERVE_STATIC=true)");
    } else {
        println!("🧭 Modo proxy: no se sirven estáticos desde la app (SERVE_STATIC=false)");
    }

    app
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
