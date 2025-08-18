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
        th { background: #f7f7f7; text-align: left; }
        .muted { color: #666; font-size: 0.9em; }
        .nowrap { white-space: nowrap; }
        .pill { display:inline-block; padding:2px 8px; border-radius: 999px; border:1px solid #ddd; font-size:.85em; }
        .pill.ok { background:#e6ffed; border-color:#b7f5c7; }
        .pill.no { background:#ffecec; border-color:#f5c1c1; }
        .btn { padding:6px 10px; }
      </style>

      <!-- ===== index ===== -->
      <h2>Resumen</h2>
      <ul id="dash-list" class="muted">
        <li>Cargando métricas…</li>
      </ul>

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
              const sales = Number(b.sales_count || 0);
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
              const sorted = [...list].sort((a, b) => (b.sales_count || 0) - (a.sales_count || 0));
              const top5Ids = new Set(sorted.slice(0, 5).map(x => x.id));
              for (const x of list) {
                top5PerYear.set(`${year}:${x.id}`, top5Ids.has(x.id));
              }
            }

            // Top 50 por ventas del libro (históricas)
            const top50 = [...books]
              .sort((a, b) => (b.sales_count || 0) - (a.sales_count || 0))
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
                <td class="nowrap">${Number(b.sales_count || 0).toLocaleString()}</td>
                <td class="nowrap">${Number(aTotal).toLocaleString()}</td>
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
        loadTopBooksByRating();
        loadTop50Sales();
      </script>
    "#;

    RawHtml(render_page("Tablas", body))
}