// src/views/reviews_form.rs
use rocket::form::{Form, FromForm};
use rocket::response::{content::RawHtml, Redirect};
use rocket::State;
use crate::Db;
use super::layout::render_page;

use crate::models::{CreateReview, UpdateReview, ReviewWithBook};
use crate::repository::reviews as reviews_repo;

#[derive(FromForm)]
pub struct ReviewForm {
    pub review_text: String,
    pub rating: i32,                 // 1..=5
    pub positive_votes: Option<i32>, // solo se usa en editar
}

#[get("/books/<book_id>/reviews/new")]
pub async fn new_review_form(book_id: i32) -> RawHtml<String> {
    // Form simple para crear
    let body = format!(r#"
        <h2>Crear Review para Libro #{book_id}</h2>
        <form method="post" action="/books/{book_id}/reviews/create">
            <label>Descripción:<br>
              <textarea name="review_text" required rows="4" cols="50"></textarea>
            </label><br><br>

            <label>Puntaje (1 a 5):
              <input type="number" name="rating" min="1" max="5" value="5" required>
            </label><br><br>

            <button type="submit">Crear</button>
            <a href="/books/{book_id}"><button type="button">Volver</button></a>
        </form>
    "#);
    RawHtml(render_page("Crear Review", &body))
}

#[post("/books/<book_id>/reviews/create", data = "<form_data>")]
pub async fn reviews_create(book_id: i32, pool: &State<Db>, form_data: Form<ReviewForm>) -> Redirect {
    let payload = CreateReview {
        book_id,
        review_text: form_data.review_text.clone(),
        rating: form_data.rating,
    };
    let _ = reviews_repo::create_review(&pool.0, &payload).await;
    Redirect::to(format!("/books/{book_id}"))
}

#[get("/books/<book_id>/reviews/<id>/edit")]
pub async fn edit_review_form(book_id: i32, id: i32, pool: &State<Db>) -> RawHtml<String> {
    // Traemos reviews del libro y buscamos la que queremos editar
    let existing: Option<ReviewWithBook> = reviews_repo::get_reviews_by_book(&pool.0, book_id)
        .await
        .ok()
        .and_then(|list| list.into_iter().find(|r| r.id == Some(id)));

    let (review_text, rating, positive_votes) = if let Some(r) = existing {
        (r.review_text, r.rating, r.positive_votes)
    } else {
        ("".to_string(), 5, 0)
    };

    let body = format!(r#"
        <h2>Editar Review #{id} (Libro #{book_id})</h2>
        <form method="post" action="/books/{book_id}/reviews/{id}/update">
            <label>Descripción:<br>
              <textarea name="review_text" required rows="4" cols="50">{review_text}</textarea>
            </label><br><br>

            <label>Puntaje (1 a 5):
              <input type="number" name="rating" min="1" max="5" value="{rating}" required>
            </label><br><br>

            <label>Up-votes:
              <input type="number" name="positive_votes" min="0" value="{positive_votes}">
            </label><br><br>

            <button type="submit">Guardar</button>
            <a href="/books/{book_id}"><button type="button">Volver</button></a>
        </form>
    "#);

    RawHtml(render_page("Editar Review", &body))
}

#[post("/books/<book_id>/reviews/<id>/update", data = "<form_data>")]
pub async fn reviews_update(book_id: i32, id: i32, pool: &State<Db>, form_data: Form<ReviewForm>) -> Redirect {
    let payload = UpdateReview {
        book_id: Some(book_id),
        review_text: Some(form_data.review_text.clone()),
        rating: Some(form_data.rating),
        positive_votes: form_data.positive_votes, // puede ser None
    };
    let _ = reviews_repo::update_review(&pool.0, id, &payload).await;
    Redirect::to(format!("/books/{book_id}"))
}