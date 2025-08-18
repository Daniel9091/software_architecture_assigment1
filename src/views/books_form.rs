use rocket::form::{Form, FromForm};
use rocket::response::{content::RawHtml, Redirect};
use rocket::State;
use crate::Db;
use super::layout::render_page;
use crate::models::{CreateBook, UpdateBook};
use crate::repository::books as books_repo; // 👈 usar repository

#[derive(FromForm)]
pub struct BookForm {
    pub title: String,
    pub summary: Option<String>,
    pub publication_date: String,
    pub author_id: i32,
}

#[get("/books/new")]
pub async fn new_book_form() -> RawHtml<String> {
    let body = r#"
        <h2>Crear Libro</h2>
        <form method="post" action="/books/create">
            <label>Título: <input type="text" name="title" required></label><br>
            <label>Resumen: <textarea name="summary"></textarea></label><br>
            <label>Fecha publicación: <input type="date" name="publication_date" required></label><br>
            <label>Autor ID: <input type="number" name="author_id" required></label><br>
            <button type="submit">Crear</button>
        </form>
        <p><a href="/books">Volver</a></p>
    "#;
    RawHtml(render_page("Nuevo Libro", body))
}

#[post("/books/create", data = "<form_data>")]
pub async fn books_create(pool: &State<Db>, form_data: Form<BookForm>) -> Redirect {
    let payload = CreateBook {
        title: form_data.title.clone(),
        summary: form_data.summary.clone(),
        publication_date: form_data.publication_date.clone(),
        author_id: form_data.author_id,
    };
    let _ = books_repo::create_book(&pool.0, &payload).await; // 👈 repository
    Redirect::to("/books")
}

#[get("/books/<id>/edit")]
pub async fn edit_book_form(id: i32, pool: &State<Db>) -> RawHtml<String> {
    let existing = books_repo::get_book_by_id(&pool.0, id).await.ok().flatten(); // 👈 repository
    let (title, summary, date, author_id) = if let Some(b) = existing {
        (
            b.title,
            b.summary.unwrap_or_default(),
            b.publication_date,
            b.author.id.unwrap_or(1),
        )
    } else {
        ("".to_string(), "".to_string(), "".to_string(), 1)
    };

    let body = format!(r#"
        <h2>Editar Libro #{id}</h2>
        <form method="post" action="/books/{id}/update">
            <label>Título: <input type="text" name="title" value="{title}" required></label><br>
            <label>Resumen: <textarea name="summary">{summary}</textarea></label><br>
            <label>Fecha publicación: <input type="date" name="publication_date" value="{date}"></label><br>
            <label>Autor ID: <input type="number" name="author_id" value="{author_id}"></label><br>
            <button type="submit">Guardar Cambios</button>
        </form>
        <p><a href="/books">Volver</a></p>
    "#);
    RawHtml(render_page("Editar Libro", &body))
}

#[post("/books/<id>/update", data = "<form_data>")]
pub async fn books_update(id: i32, pool: &State<Db>, form_data: Form<BookForm>) -> Redirect {
    let payload = UpdateBook {
        title: Some(form_data.title.clone()),
        summary: form_data.summary.clone(),
        publication_date: Some(form_data.publication_date.clone()),
        author_id: Some(form_data.author_id),
    };
    let _ = books_repo::update_book(&pool.0, id, &payload).await; // 👈 repository
    Redirect::to("/books")
}