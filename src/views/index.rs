use rocket::response::Redirect;

#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to(uri!(crate::views::books::books_index))
}
