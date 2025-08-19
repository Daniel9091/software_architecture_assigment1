use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/tables")]
pub async fn tables_index(_pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <style>
        table { border-collapse: collapse; width: 100%; margin-top: 1rem; }
        th, td { border: 1px solid #ddd; padding: 8px; vertical-align: top; }
        th { background: #f7f7f7; text-align: left; cursor: default; }
        .th-sortable { cursor: pointer; user-select: none; }
        .muted { color: #666; font-size: 0.9em; }
        .nowrap { white-space: nowrap; }
        .pill { display:inline-block; padding:2px 8px; border-radius: 999px; border:1px solid #ddd; font-size:.85em; }
        .pill.ok { background:#e6ffed; border-color:#b7f5c7; }
        .pill.no { background:#ffecec; border-color:#f5c1c1; }
        .btn { padding:6px 10px; }
        .filters input { width: 100%; box-sizing: border-box; padding: 4px; }
        .right { text-align: right; }
        .center { text-align: center; }
      </style>

      <!-- ===== index ===== -->
      <h2>Resumen</h2>
      <ul id="dash-list" class="muted">
        <li>Cargando métricas…</li>
      </ul>

      <!-- ===== authors agg ===== -->
      <h2>Autores — libros, puntaje promedio y ventas totales</h2>
      <table>
        <thead>
          <tr>
            <th class="th-sortable" data-sort-key="author">Autor</th>
            <th class="th-sortable nowrap" data-sort-key="books"># Libros</th>
            <th class="th-sortable nowrap" data-sort-key="avg">Rating prom.</th>
            <th class="th-sortable nowrap right" data-sort-key="sales">Ventas totales</th>
          </tr>
          <tr class="filters">
            <th><input id="f-author" placeholder="Filtrar autor…"></th>
            <th><input id="f-books" placeholder="≥ libros…"></th>
            <th><input id="f-avg" placeholder="≥ rating…"></th>
            <th><input id="f-sales" placeholder="≥ ventas…"></th>
          </tr>
        </thead>
        <tbody id="authors-body">
          <tr><td colspan="4" class="muted">Calculando…</td></tr>
        </tbody>
      </table>

      <!-- ===== top 10 ===== -->
      <h2>Top 10 libros por rating promedio</h2>
      <table>
        <thead>
          <tr>
            <th class="nowrap">Libro</th>
            <th class="nowrap">Rating promedio</th>
            <th>Review más alta (más popular)</th>
            <th>Review más baja (más popular)</th>
            <th class="nowrap">Ver</th>
          </tr>
        </thead>
        <tbody id="top-books-body">
          <tr><td colspan="5" class="muted">Calculando…</td></tr>
        </tbody>
      </table>

      <!-- ===== 50 sales ===== -->
      <h2>Top 50 libros por ventas históricas</h2>
      <table>
        <thead>
          <tr>
            <th>Libro</th>
            <th class="nowrap">Ventas libro</th>
            <th class="nowrap">Ventas totales autor</th>
            <th class="nowrap">Top 5 en su año</th>
            <th class="nowrap">Ver</th>
          </tr>
        </thead>
        <tbody id="top-sales-body">
          <tr><td colspan="5" class="muted">Calculando…</td></tr>
        </tbody>
      </table>

      <script>
        function truncate(text, len = 140) {
          if (!text) return '';
          text = String(text);
          return text.length > len ? text.slice(0, len - 1) + '…' : text;
        }
        function num(x){ const n = Number(x); return Number.isFinite(n) ? n : 0; }

        // ===== index =====
        async function loadDashboard() {
          const ul = document.getElementById('dash-list');
          try {
            const res = await fetch('/api/dashboard');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error(wrapper?.message || 'Respuesta inválida');
            const d = wrapper.data || wrapper;

            ul.innerHTML = `
              <li><strong>average_rating:</strong> ${Number(d.average_rating ?? 0).toFixed(6)}</li>
              <li><strong>total_authors:</strong> ${d.total_authors ?? 0}</li>
              <li><strong>total_books:</strong> ${d.total_books ?? 0}</li>
              <li><strong>total_reviews:</strong> ${d.total_reviews ?? 0}</li>
              <li><strong>total_sales:</strong> ${d.total_sales ?? 0}</li>
            `;
          } catch (err) {
            ul.innerHTML = `<li>Error cargando métricas: ${err}</li>`;
          }
        }

        // ===== authors agg =====
        const authorsAggState = {
          rows: [],
          sortKey: 'sales',
          sortDir: 'desc' // 'asc'|'desc'
        };

        async function loadAuthorsAgg() {
          const tbody = document.getElementById('authors-body');
          try {
            // Traemos libros (traen autor embebido + sales_count)
            const booksRes = await fetch('/api/books');
            if (!booksRes.ok) throw new Error(booksRes.statusText);
            const booksWrapper = await booksRes.json();
            if (!booksWrapper.success) throw new Error(booksWrapper.message);
            const books = Array.isArray(booksWrapper.data) ? booksWrapper.data : [];

            // Mapa por autor
            const byAuthor = new Map(); // author_id -> { author_id, author_name, books:[], salesTotal, ratings:[] }
            for (const b of books) {
              const aid = b.author?.id ?? b.author_id;
              const aname = b.author?.name ?? 'Desconocido';
              if (!byAuthor.has(aid)) {
                byAuthor.set(aid, { author_id: aid, author_name: aname, books: [], salesTotal: 0, ratings: [] });
              }
              const entry = byAuthor.get(aid);
              entry.books.push(b);
              entry.salesTotal += num(b.sales_count);
            }

            // Para calcular rating promedio del autor, juntamos todas las reviews de todos sus libros
            // (simple pero potencialmente costoso. Para datasets medianos funciona bien)
            const rows = [];
            for (const entry of byAuthor.values()) {
              let ratings = [];
              for (const bk of entry.books) {
                try {
                  const rRes = await fetch(`/api/books/${bk.id}/reviews`);
                  if (!rRes.ok) throw new Error(rRes.statusText);
                  const rWrap = await rRes.json();
                  if (!rWrap.success) throw new Error(rWrap.message);
                  const reviews = Array.isArray(rWrap.data) ? rWrap.data : [];
                  ratings.push(...reviews.map(r => num(r.rating)));
                } catch (e) {
                  console.warn('No se pudieron leer reviews de libro', bk?.id, e);
                }
              }
              const avg = ratings.length ? (ratings.reduce((a, b) => a + b, 0) / ratings.length) : 0;
              rows.push({
                author_id: entry.author_id,
                author_name: entry.author_name,
                books: entry.books.length,
                avg,
                sales: entry.salesTotal
              });
            }

            authorsAggState.rows = rows;
            renderAuthorsAgg();
            wireAuthorsSortAndFilters();
          } catch (err) {
            tbody.innerHTML = `<tr><td colspan="4">Error calculando tabla de autores: ${err}</td></tr>`;
          }
        }

        function applyAuthorsFilters(data) {
          const fa = document.getElementById('f-author').value.trim().toLowerCase();
          const fb = document.getElementById('f-books').value.trim();
          const favg = document.getElementById('f-avg').value.trim();
          const fs = document.getElementById('f-sales').value.trim();

          return data.filter(r => {
            if (fa && !String(r.author_name).toLowerCase().includes(fa)) return false;
            if (fb && !(r.books >= Number(fb))) return false;
            if (favg && !(r.avg >= Number(favg))) return false;
            if (fs && !(r.sales >= Number(fs))) return false;
            return true;
          });
        }

        function sortByKey(data, key, dir) {
          const mul = dir === 'desc' ? -1 : 1;
          return [...data].sort((a, b) => {
            let va = a[key], vb = b[key];
            if (typeof va === 'string') { va = va.toLowerCase(); vb = String(vb).toLowerCase(); }
            if (va < vb) return -1 * mul;
            if (va > vb) return 1 * mul;
            return 0;
          });
        }

        function renderAuthorsAgg() {
          const tbody = document.getElementById('authors-body');
          const state = authorsAggState;

          let rows = applyAuthorsFilters(state.rows);
          rows = sortByKey(rows, state.sortKey, state.sortDir);

          tbody.innerHTML = '';
          if (rows.length === 0) {
            tbody.innerHTML = '<tr><td colspan="4" class="muted">No hay datos.</td></tr>';
            return;
          }

          for (const r of rows) {
            const tr = document.createElement('tr');
            tr.innerHTML = `
              <td>${r.author_name}</td>
              <td class="center">${r.books}</td>
              <td class="center">${r.avg.toFixed(2)}</td>
              <td class="right">${Number(r.sales).toLocaleString()}</td>
            `;
            tbody.appendChild(tr);
          }
        }

        function wireAuthorsSortAndFilters() {
          // Sorting
          document.querySelectorAll('th.th-sortable').forEach(th => {
            th.addEventListener('click', () => {
              const key = th.dataset.sortKey;
              if (!key) return;
              if (authorsAggState.sortKey === key) {
                authorsAggState.sortDir = authorsAggState.sortDir === 'asc' ? 'desc' : 'asc';
              } else {
                authorsAggState.sortKey = key;
                authorsAggState.sortDir = key === 'author' ? 'asc' : 'desc';
              }
              renderAuthorsAgg();
            });
          });
          // Filters
          ['f-author','f-books','f-avg','f-sales'].forEach(id => {
            const el = document.getElementById(id);
            el.addEventListener('input', () => renderAuthorsAgg());
          });
        }

        // ===== top 10 =====
        async function loadTopBooksByRating() {
          const tbody = document.getElementById('top-books-body');
          try {
            const booksRes = await fetch('/api/books');
            if (!booksRes.ok) throw new Error(booksRes.statusText);
            const booksWrapper = await booksRes.json();
            if (!booksWrapper.success) throw new Error(booksWrapper.message);
            const books = Array.isArray(booksWrapper.data) ? booksWrapper.data : [];

            const withStats = [];
            for (const b of books) {
              try {
                const rRes = await fetch(`/api/books/${b.id}/reviews`);
                if (!rRes.ok) throw new Error(rRes.statusText);
                const rWrap = await rRes.json();
                if (!rWrap.success) throw new Error(rWrap.message);
                const reviews = Array.isArray(rWrap.data) ? rWrap.data : [];
                if (reviews.length === 0) continue;

                const avg = reviews.reduce((acc, r) => acc + (r.rating || 0), 0) / reviews.length;

                const maxRating = Math.max(...reviews.map(r => r.rating || 0));
                const highest = reviews.filter(r => (r.rating || 0) === maxRating)
                  .sort((a, b) => (b.positive_votes || 0) - (a.positive_votes || 0))[0];

                const minRating = Math.min(...reviews.map(r => r.rating || 0));
                const lowest = reviews.filter(r => (r.rating || 0) === minRating)
                  .sort((a, b) => (b.positive_votes || 0) - (a.positive_votes || 0))[0];

                withStats.push({ id: b.id, title: b.title, avgRating: avg, highest, lowest });
              } catch (e) {
                console.warn('No se pudieron calcular reviews para libro', b?.id, e);
              }
            }

            withStats.sort((a, b) => b.avgRating - a.avgRating);
            const top10 = withStats.slice(0, 10);

            tbody.innerHTML = '';
            if (top10.length === 0) {
              tbody.innerHTML = '<tr><td colspan="5" class="muted">No hay datos suficientes de reviews.</td></tr>';
              return;
            }

            for (const item of top10) {
              const h = item.highest || {};
              const l = item.lowest || {};
              const tr = document.createElement('tr');
              tr.innerHTML = `
                <td>
                  <div><strong>${truncate(item.title, 80)}</strong></div>
                  <div class="muted">ID: ${item.id}</div>
                </td>
                <td class="nowrap">${item.avgRating.toFixed(2)}</td>
                <td>
                  <div><strong>Rating:</strong> ${h.rating ?? ''} <span class="muted">(${h.positive_votes ?? 0} up-votes)</span></div>
                  <div>${truncate(h.review_text, 240)}</div>
                </td>
                <td>
                  <div><strong>Rating:</strong> ${l.rating ?? ''} <span class="muted">(${l.positive_votes ?? 0} up-votes)</span></div>
                  <div>${truncate(l.review_text, 240)}</div>
                </td>
                <td class="nowrap">
                  <a href="/books/${item.id}"><button class="btn">Ver libro</button></a>
                </td>
              `;
              tbody.appendChild(tr);
            }
          } catch (err) {
            tbody.innerHTML = `<tr><td colspan="5">Error calculando top 10: ${err}</td></tr>`;
          }
        }

        // ===== 50 sales =====
        async function loadTop50Sales() {
          const tbody = document.getElementById('top-sales-body');
          try {
            const booksRes = await fetch('/api/books');
            if (!booksRes.ok) throw new Error(booksRes.statusText);
            const booksWrapper = await booksRes.json();
            if (!booksWrapper.success) throw new Error(booksWrapper.message);
            const books = Array.isArray(booksWrapper.data) ? booksWrapper.data : [];

            // Totales por autor
            const authorTotals = new Map(); // author_id -> total_sales
            for (const b of books) {
              const aid = b.author?.id ?? b.author_id;
              const sales = num(b.sales_count);
              if (aid != null) authorTotals.set(aid, (authorTotals.get(aid) || 0) + sales);
            }

            // Agrupar por año de publicación para calcular si está en top 5 de su año
            const byYear = new Map(); // year -> books[]
            for (const b of books) {
              const year = (b.publication_date || '').slice(0, 4) || 'N/A';
              if (!byYear.has(year)) byYear.set(year, []);
              byYear.get(year).push(b);
            }

            // Precalcular rankings por año (orden por ventas del libro)
            const top5PerYear = new Map(); // key: `${year}:${book_id}` -> boolean
            for (const [year, list] of byYear.entries()) {
              const sorted = [...list].sort((a, b) => num(b.sales_count) - num(a.sales_count));
              const top5Ids = new Set(sorted.slice(0, 5).map(x => x.id));
              for (const x of list) {
                top5PerYear.set(`${year}:${x.id}`, top5Ids.has(x.id));
              }
            }

            // Top 50 por ventas del libro (históricas)
            const top50 = [...books]
              .sort((a, b) => num(b.sales_count) - num(a.sales_count))
              .slice(0, 50);

            tbody.innerHTML = '';
            if (top50.length === 0) {
              tbody.innerHTML = '<tr><td colspan="5" class="muted">No hay datos de ventas.</td></tr>';
              return;
            }

            for (const b of top50) {
              const aid = b.author?.id ?? b.author_id;
              const aTotal = aid != null ? (authorTotals.get(aid) || 0) : 0;
              const year = (b.publication_date || '').slice(0, 4) || 'N/A';
              const isTop5Year = top5PerYear.get(`${year}:${b.id}`) === true;

              const tr = document.createElement('tr');
              tr.innerHTML = `
                <td>
                  <div><strong>${truncate(b.title, 80)}</strong></div>
                  <div class="muted">ID: ${b.id} · Autor: ${b.author?.name ?? 'Desconocido'} · Año: ${year}</div>
                </td>
                <td class="nowrap">${num(b.sales_count).toLocaleString()}</td>
                <td class="nowrap">${num(aTotal).toLocaleString()}</td>
                <td class="nowrap">
                  <span class="pill ${isTop5Year ? 'ok' : 'no'}">${isTop5Year ? 'Sí' : 'No'}</span>
                </td>
                <td class="nowrap">
                  <a href="/books/${b.id}"><button class="btn">Ver libro</button></a>
                </td>
              `;
              tbody.appendChild(tr);
            }
          } catch (err) {
            tbody.innerHTML = `<tr><td colspan="5">Error calculando top 50: ${err}</td></tr>`;
          }
        }

        // Cargas en paralelo
        loadDashboard();
        loadAuthorsAgg();
        loadTopBooksByRating();
        loadTop50Sales();
      </script>
    "#;

    RawHtml(render_page("Tablas", body))
}