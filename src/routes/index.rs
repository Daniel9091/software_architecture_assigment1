use rocket::response::Redirect;

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to(uri!(crate::routes::books::books_index))
}
