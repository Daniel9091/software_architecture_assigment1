use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::State;
use crate::Db;

/// HTML de la barra inferior compartida
fn bottom_nav_html() -> String {
    r#"
    <nav style="position:fixed;bottom:0;left:0;right:0;height:56px;background:#222;color:#fff;display:flex;justify-content:space-around;align-items:center;">
        <a href="/authors" style="color:#fff;text-decoration:none;padding:8px 16px;">Autores</a>
        <a href="/books"   style="color:#fff;text-decoration:none;padding:8px 16px;">Libros</a>
        <a href="/tables"  style="color:#fff;text-decoration:none;padding:8px 16px;">Tablas</a>
    </nav>
    "#
    .to_string()
}

/// Wrapper que inserta el contenido dentro de un layout básico + barra inferior
fn render_page(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="es">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width,initial-scale=1"/>
  <title>{}</title>
  <style>
    body {{ margin:0 0 76px 0; font-family:Arial,Helvetica,sans-serif; padding:16px; }}
    .container {{ max-width:900px; margin:0 auto; }}
    ul {{ padding-left:18px; }}
    .loading {{ color:#666; }}
  </style>
</head>
<body>
  <div class="container">
    <h1>{}</h1>
    {}
  </div>
  {}
</body>
</html>"#,
        title,
        title,
        body,
        bottom_nav_html()
    )
}

/// raiz: redirige a /books
#[get("/")]
pub async fn index() -> Redirect {
    Redirect::to(uri!(books_index))
}

/// Página de índice de libros (usa /api/books para poblar contenido)
#[get("/books")]
pub async fn books_index(_pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <p class="loading">Cargando libros...</p>
      <ul id="list"></ul>
      <script>
        async function load() {
          try {
            const res = await fetch('/api/books');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');
            const data = wrapper.data || [];
            const list = document.getElementById('list');
            list.innerHTML = '';
            if (!Array.isArray(data) || data.length === 0) {
              list.innerHTML = '<li>No hay libros</li>';
              document.querySelector('.loading').style.display = 'none';
              return;
            }
            for (const b of data) {
              const li = document.createElement('li');
              const authorName = (b.author && b.author.name) ? b.author.name : (b.author_name || 'Autor desconocido');
              li.textContent = (b.title ? b.title : JSON.stringify(b)) + ' — ' + authorName;
              list.appendChild(li);
            }
            document.querySelector('.loading').style.display = 'none';
          } catch (err) {
            document.querySelector('.loading').textContent = 'Error cargando libros: ' + err;
          }
        }
        load();
      </script>
    "#;
    RawHtml(render_page("Libros", body))
}

/// Página índice de autores (usa /api/authors)
#[get("/authors")]
pub async fn authors_index(_pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <p class="loading">Cargando autores...</p>
      <ul id="list"></ul>
      <script>
        async function load() {
          try {
            const res = await fetch('/api/authors');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');
            const data = wrapper.data || [];
            const list = document.getElementById('list');
            list.innerHTML = '';
            if (!Array.isArray(data) || data.length === 0) {
              list.innerHTML = '<li>No hay autores</li>';
              document.querySelector('.loading').style.display = 'none';
              return;
            }
            for (const a of data) {
              const li = document.createElement('li');
              li.textContent = a.name ?? JSON.stringify(a);
              list.appendChild(li);
            }
            document.querySelector('.loading').style.display = 'none';
          } catch (err) {
            document.querySelector('.loading').textContent = 'Error cargando autores: ' + err;
          }
        }
        load();
      </script>
    "#;
    RawHtml(render_page("Autores", body))
}

/// Página índice de tablas (lista tablas o información útil desde /api/tables)
#[get("/tables")]
pub async fn tables_index(_pool: &State<Db>) -> RawHtml<String> {
    // la API expone estadísticas en /api/dashboard, usar eso aquí
    let body = r#"
      <p class="loading">Cargando tablas...</p>
      <pre id="info"></pre>
      <script>
        async function load() {
          try {
            const res = await fetch('/api/dashboard');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');
            const data = wrapper.data || wrapper;
            document.getElementById('info').textContent = JSON.stringify(data, null, 2);
            document.querySelector('.loading').style.display = 'none';
          } catch (err) {
            document.querySelector('.loading').textContent = 'Error cargando tablas: ' + err;
          }
        }
        load();
      </script>
    "#;
    RawHtml(render_page("Tablas", body))
}
