use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

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


