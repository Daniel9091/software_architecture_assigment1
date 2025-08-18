use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/authors")]
pub async fn authors_index(_pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <style>
        ul#list { list-style: none; padding: 0; }
        ul#list li { display: flex; gap: .5rem; align-items: center; margin: .25rem 0; }
        .btn { padding: .25rem .5rem; border: 1px solid #ccc; border-radius: .375rem; background: #f7f7f7; cursor: pointer; }
        .btn:hover { background: #eee; }
      </style>
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

              const name = document.createElement('span');
              name.textContent = a.name ?? '(sin nombre)';

              const btn = document.createElement('a');
              btn.className = 'btn';
              btn.textContent = 'Ver';
              // link al show SSR:
              btn.href = `/authors/${a.id}`;

              li.appendChild(name);
              li.appendChild(btn);
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


#[get("/authors/<id>")]
pub async fn authors_show(_pool: &State<Db>, id: i32) -> RawHtml<String> {
    let body = format!(r#"
      <style>
        .meta p {{ margin: .25rem 0; }}
        .books ul {{ padding-left: 1rem; }}
      </style>

      <p class="loading">Cargando autor...</p>
      <section class="meta" style="display:none">
        <h1 id="name"></h1>
        <p><strong>País:</strong> <span id="country">—</span></p>
        <p><strong>Nacimiento:</strong> <span id="birth">—</span></p>
        <p><strong>Descripción:</strong><br><span id="desc">—</span></p>
      </section>

      <section class="books" style="display:none">
        <h2>Libros</h2>
        <ul id="books"></ul>
      </section>

      <p><a href="/authors">← Volver a autores</a></p>

      <script>
        async function load() {{
          try {{
            const res = await fetch('/api/authors/{id}/details');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');

            const details = wrapper.data;
            const a = details.author || {{}};
            document.getElementById('name').textContent = a.name ?? '(sin nombre)';
            document.getElementById('country').textContent = a.country ?? '—';
            document.getElementById('birth').textContent = a.birth_date ?? '—';
            document.getElementById('desc').textContent = a.description ?? '—';
            document.querySelector('.meta').style.display = '';

            const books = details.books || [];
            const list = document.getElementById('books');
            list.innerHTML = '';
            if (Array.isArray(books) && books.length) {{
              for (const b of books) {{
                const li = document.createElement('li');
                const year = b.publication_date ? ' (' + b.publication_date + ')' : '';
                li.innerHTML = '<a href="/books/' + b.id + '">' + b.title + '</a>' + year;
                list.appendChild(li);
              }}
            }} else {{
              list.innerHTML = '<li>Este autor aún no tiene libros.</li>';
            }}
            document.querySelector('.books').style.display = '';
            document.querySelector('.loading').style.display = 'none';
          }} catch (err) {{
            document.querySelector('.loading').textContent = 'Error cargando autor: ' + err;
          }}
        }}
        load();
      </script>
    "#);
    RawHtml(render_page("Autor", &body))
}
