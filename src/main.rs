#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("sqlite_db")]
struct Db(sqlx::SqlitePool);

#[get("/")]
async fn index() -> &'static str {
    "Hola, Rocket + SQLite!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
