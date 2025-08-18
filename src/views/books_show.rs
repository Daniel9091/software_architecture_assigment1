use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/books/<_id>")]
pub async fn books_show(_id: i32, _pool: &State<Db>) -> RawHtml<String> {
    // No usamos `format!` para no pelear con `${...}` de JS.
    let body = r#"
      <div id="book-details">
        <p class="loading">Cargando libro...</p>
      </div>

      <div style="margin:12px 0;">
        <button id="btn-back">Volver</button>
        <button id="btn-edit">Editar libro</button>
        <button id="btn-delete">Eliminar libro</button>
      </div>

      <h2>Reseñas</h2>
      <div style="margin:8px 0;">
        <button id="btn-new-review">Crear review</button>
      </div>

      <table border="1" style="margin-top:1em; border-collapse: collapse; width: 100%;">
        <thead>
          <tr>
            <th>Descripción</th>
            <th>Puntaje</th>
            <th>Up-votes</th>
            <th>Acciones</th>
          </tr>
        </thead>
        <tbody id="reviews-body">
          <tr><td colspan="4" class="loading">Cargando reseñas...</td></tr>
        </tbody>
      </table>

      <script>
        // Tomamos el id desde la URL /books/:id
        const segments = window.location.pathname.split('/');
        const bookId = segments[segments.length - 1];

        // Navegación básica
        document.getElementById('btn-back').onclick = () => location.href = '/books';
        document.getElementById('btn-edit').onclick = () => location.href = `/books/${bookId}/edit`;
        document.getElementById('btn-delete').onclick = deleteBook;
        document.getElementById('btn-new-review').onclick = () => location.href = `/books/${bookId}/reviews/new`;

        async function loadBook() {
          try {
            const res = await fetch(`/api/books/${bookId}`);
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper.success) throw new Error(wrapper.message);
            const book = wrapper.data;

            const authorName = (book.author && book.author.name) || book.author_name || 'Desconocido';
            document.getElementById('book-details').innerHTML = `
              <p><b>Título:</b> ${book.title}</p>
              <p><b>Autor:</b> ${authorName}</p>
              <p><b>Resumen:</b> ${book.summary ?? ''}</p>
              <p><b>Fecha de publicación:</b> ${book.publication_date || ''}</p>
              <p><b>Número de ventas:</b> ${book.sales_count ?? 0}</p>
            `;
          } catch (err) {
            document.getElementById('book-details').innerHTML = 'Error cargando libro: ' + err;
          }
        }

        async function loadReviews() {
          try {
            const res = await fetch(`/api/books/${bookId}/reviews`);
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper.success) throw new Error(wrapper.message);

            const reviews = wrapper.data || [];
            const tbody = document.getElementById('reviews-body');
            tbody.innerHTML = '';

            if (!Array.isArray(reviews) || reviews.length === 0) {
              tbody.innerHTML = '<tr><td colspan="4">No hay reseñas</td></tr>';
              return;
            }

            for (const r of reviews) {
              const tr = document.createElement('tr');
              tr.innerHTML = `
                <td>${r.review_text || ''}</td>
                <td>${r.rating || 0}</td>
                <td>${r.positive_votes ?? 0}</td>
                <td>
                  <button onclick="location.href='/books/${bookId}/reviews/${r.id}/edit'">Editar</button>
                  <button onclick="deleteReview(${r.id})">Eliminar</button>
                </td>
              `;
              tbody.appendChild(tr);
            }
          } catch (err) {
            document.getElementById('reviews-body').innerHTML =
              '<tr><td colspan="4">Error cargando reseñas: ' + err + '</td></tr>';
          }
        }

        async function deleteBook() {
          if (!confirm('¿Seguro que deseas eliminar este libro?')) return;
          try {
            const res = await fetch(`/api/books/${bookId}`, { method: 'DELETE' });
            const data = await res.json().catch(() => ({}));
            if (!res.ok || (data && data.success === false)) {
              throw new Error((data && data.message) || res.statusText);
            }
            // volver al listado
            location.href = '/books';
          } catch (err) {
            alert('Error eliminando libro: ' + err);
          }
        }

        async function deleteReview(rid) {
          if (!confirm('¿Seguro que deseas eliminar esta reseña?')) return;
          try {
            const res = await fetch(`/api/reviews/${rid}`, { method: 'DELETE' });
            const data = await res.json().catch(() => ({}));
            if (!res.ok || (data && data.success === false)) {
              throw new Error((data && data.message) || res.statusText);
            }
            loadReviews();
          } catch (err) {
            alert('Error eliminando reseña: ' + err);
          }
        }

        // Carga inicial
        loadBook();
        loadReviews();
      </script>
    "#;

    RawHtml(render_page("Detalle del libro", body))
}