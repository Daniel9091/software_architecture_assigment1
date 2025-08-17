use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/tables")]
pub async fn tables_index(_pool: &State<Db>) -> RawHtml<String> {
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
