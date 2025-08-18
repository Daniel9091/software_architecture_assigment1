use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/books")]
pub async fn books_index(_pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <!-- Search bar -->
      <form action="/books/search" method="get" style="margin-bottom: 1em;">
        <input type="text" name="q" id="searchInput" placeholder="Buscar libros..." />
        <button type="submit">Buscar</button>
      </form>

      <a href="/books/new">
        <button style="margin-bottom: 1em;">Crear libro</button>
      </a>

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
              li.innerHTML = (b.title ? b.title : JSON.stringify(b)) + ' — ' + authorName
                + ` <a href="/books/${b.id}"><button>Ver</button></a>`;
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
