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
      </style>

      <p class="loading">Cargando tablas...</p>
      <pre id="info" class="muted"></pre>

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

      <script>
        function truncate(text, len = 140) {
          if (!text) return '';
          text = String(text);
          return text.length > len ? text.slice(0, len - 1) + '…' : text;
        }

        async function loadDashboard() {
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

        async function loadTopBooks() {
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
                  <a href="/books/${item.id}"><button>Ver libro</button></a>
                </td>
              `;
              tbody.appendChild(tr);
            }
          } catch (err) {
            tbody.innerHTML = `<tr><td colspan="5">Error calculando top 10: ${err}</td></tr>`;
          }
        }

        loadDashboard();
        loadTopBooks();
      </script>
    "#;
    RawHtml(render_page("Tablas", body))
}