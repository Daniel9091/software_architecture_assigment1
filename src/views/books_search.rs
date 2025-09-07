// src/views/books_search.rs
use rocket::response::content::RawHtml;
use super::layout::render_page;

#[get("/books/search")]
pub async fn books_search_page() -> RawHtml<String> {
    let body = r#"
      <style>
        table { border-collapse: collapse; width: 100%; margin-top: 1rem; }
        th, td { border: 1px solid #ddd; padding: 8px; vertical-align: top; }
        th { background: #f7f7f7; text-align: left; }
        .muted { color: #666; font-size: 0.9em; }
        .nowrap { white-space: nowrap; }
        .pager { margin-top: 1rem; display: flex; gap: .5rem; align-items: center; }
        .pager button { padding: .4rem .7rem; }
        .row-actions { white-space: nowrap; }
      </style>

      <form id="search-form" onsubmit="doSearch(1); return false;">
        <input id="q" name="q" type="text" placeholder="Palabras a buscar en la descripción..." style="width: 60%;" />
        <button type="submit">Buscar</button>
      </form>

      <div id="meta" class="muted" style="margin-top:.5rem;"></div>

      <table>
        <thead>
          <tr>
            <th>Título</th>
            <th>Autor</th>
            <th>Publicación</th>
            <th>Resumen</th>
            <th>Ventas</th>
            <th>Acciones</th>
          </tr>
        </thead>
        <tbody id="results">
          <tr><td colspan="6" class="muted">Ingresa texto y presiona Buscar…</td></tr>
        </tbody>
      </table>

      <div class="pager">
        <button id="prev" disabled>Anterior</button>
        <span id="pageinfo" class="muted"></span>
        <button id="next" disabled>Siguiente</button>
      </div>

      <script>
        const perPage = 10;
        const results = document.getElementById('results');
        const meta = document.getElementById('meta');
        const prevBtn = document.getElementById('prev');
        const nextBtn = document.getElementById('next');
        const pageInfo = document.getElementById('pageinfo');
        const qInput = document.getElementById('q');

        function truncate(text, len = 140) {
          if (!text) return '';
          text = String(text);
          return text.length > len ? text.slice(0, len - 1) + '…' : text;
        }

        function setPager(page, perPage, total) {
          const maxPage = Math.max(1, Math.ceil(total / perPage));
          prevBtn.disabled = page <= 1;
          nextBtn.disabled = page >= maxPage;
          pageInfo.textContent = `Página ${page} de ${maxPage} (total ${total})`;

          prevBtn.onclick = () => doSearch(page - 1);
          nextBtn.onclick = () => doSearch(page + 1);
        }

        async function doSearch(page = 1) {
          const q = qInput.value.trim();
          if (!q) {
            results.innerHTML = '<tr><td colspan="6" class="muted">Ingresa texto y presiona Buscar…</td></tr>';
            meta.textContent = '';
            setPager(1, perPage, 0);
            return;
          }

          results.innerHTML = '<tr><td colspan="6" class="muted">Buscando…</td></tr>';
          meta.textContent = '';

          try {
            const url = `/api/books/search?q=${encodeURIComponent(q)}&page=${page}&per_page=${perPage}`;
            const res = await fetch(url);
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');

            const { items, total, page: p, per_page: pp, query } = wrapper.data || {};
            meta.textContent = `Consulta: "${query}" — resultados: ${total}`;

            results.innerHTML = '';
            if (!items || items.length === 0) {
              results.innerHTML = '<tr><td colspan="6" class="muted">Sin resultados</td></tr>';
            } else {
              for (const b of items) {
                const authorName = (b.author && b.author.name) || 'Autor desconocido';
                const tr = document.createElement('tr');
                tr.innerHTML = `
                  <td>${b.title || ''}</td>
                  <td>${authorName}</td>
                  <td class="nowrap">${b.publication_date || ''}</td>
                  <td>${truncate(b.summary || '', 160)}</td>
                  <td class="nowrap">${b.sales_count ?? 0}</td>
                  <td class="row-actions">
                    <a href="/books/${b.id}"><button>Ver</button></a>
                  </td>
                `;
                results.appendChild(tr);
              }
            }

            setPager(p || page, pp || perPage, total || 0);
          } catch (err) {
            results.innerHTML = `<tr><td colspan="6">Error: ${err}</td></tr>`;
            setPager(1, perPage, 0);
          }
        }

        // Si viene con ?q=... en la URL, precargar
        (function boot() {
          const params = new URLSearchParams(location.search);
          const q0 = params.get('q') || '';
          if (q0) {
            qInput.value = q0;
            const p0 = parseInt(params.get('page') || '1', 10) || 1;
            doSearch(p0);
          }
        })();
      </script>
    "#;

    RawHtml(render_page("Búsqueda de libros por descripción", body))
}