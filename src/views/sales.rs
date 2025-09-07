use rocket::response::content::RawHtml;
use rocket::State;
use crate::Db;
use super::layout::render_page;

#[get("/books/<book_id>/sales")]
pub async fn sales_by_book(book_id: i32, _pool: &State<Db>) -> RawHtml<String> {
    // Usamos format! solo para inyectar book_id en una constante JS. Evitamos template strings.
    let body = format!(r#"
      <style>
        .toolbar {{ display:flex; align-items:center; gap:.5rem; margin: .5rem 0 1rem; }}
        table {{ border-collapse: collapse; width: 100%; max-width: 720px; }}
        th, td {{ border: 1px solid #ddd; padding: .5rem; text-align: left; }}
        th {{ background: #f7f7f7; }}
        .btn {{ padding: .25rem .5rem; border: 1px solid #ccc; border-radius: .375rem; background: #f7f7f7; cursor: pointer; text-decoration:none; color:inherit; }}
        .btn:hover {{ background: #eee; }}
        .danger {{ color:#b00020; }}
        .muted {{ color:#666; font-size:.9rem; }}
        input[type='number'] {{ width: 120px; padding:.25rem; }}
      </style>

      <div class="toolbar">
        <a class="btn" href="/books/{book_id}">← Volver al libro</a>
        <span id="bookTitle" class="muted"></span>
      </div>

      <h3>Agregar registro</h3>
      <form id="addForm">
        <label>Año: <input id="year" type="number" min="0" required></label>
        <label style="margin-left:.5rem;">Ventas: <input id="sales" type="number" min="0" required></label>
        <button type="submit" class="btn">Agregar</button>
        <span id="msg" class="muted"></span>
      </form>

      <h3 style="margin-top:1rem;">Historial</h3>
      <table id="tbl">
        <thead>
          <tr>
            <th style="width:140px;">Año</th>
            <th style="width:180px;">Ventas</th>
            <th>Acciones</th>
          </tr>
        </thead>
        <tbody id="tbody">
          <tr><td colspan="3" class="muted">Cargando…</td></tr>
        </tbody>
      </table>

      <script>
        const bookId = {book_id};
        const tbody = document.getElementById('tbody');
        const addForm = document.getElementById('addForm');
        const msg = document.getElementById('msg');
        const bookTitleEl = document.getElementById('bookTitle');

        async function load() {{
          const url = '/api/books/' + bookId + '/sales';
          const res = await fetch(url);
          if (!res.ok) throw new Error(res.statusText);
          const wrap = await res.json();
          if (!wrap || !wrap.success) throw new Error((wrap && wrap.message) || 'Respuesta inválida');
          const rows = wrap.data || [];
          // book_title viene repetido en cada fila; lo tomamos de la primera si existe
          if (rows.length > 0 && rows[0].book_title) {{
            bookTitleEl.textContent = 'Libro: ' + rows[0].book_title;
          }} else {{
            bookTitleEl.textContent = '';
          }}

          render(rows);
        }}

        function render(rows) {{
          tbody.innerHTML = '';
          if (!Array.isArray(rows) || rows.length === 0) {{
            tbody.innerHTML = '<tr><td colspan="3" class="muted">Sin registros</td></tr>';
            return;
          }}
          for (const r of rows) {{
            const tr = document.createElement('tr');
            tr.dataset.id = r.id;

            const tdYear = document.createElement('td');
            tdYear.textContent = r.year;

            const tdSales = document.createElement('td');
            tdSales.textContent = r.sales;

            const tdAct = document.createElement('td');

            const editBtn = document.createElement('button');
            editBtn.className = 'btn';
            editBtn.textContent = 'Editar';
            editBtn.addEventListener('click', () => enterEdit(tr, r));

            const delBtn = document.createElement('button');
            delBtn.className = 'btn danger';
            delBtn.textContent = 'Eliminar';
            delBtn.addEventListener('click', () => delRow(r.id));

            tdAct.appendChild(editBtn);
            tdAct.appendChild(delBtn);

            tr.appendChild(tdYear);
            tr.appendChild(tdSales);
            tr.appendChild(tdAct);
            tbody.appendChild(tr);
          }}
        }}

        function enterEdit(tr, r) {{
          tr.innerHTML = '';
          const tdYear = document.createElement('td');
          const inpY = document.createElement('input');
          inpY.type = 'number';
          inpY.min = '0';
          inpY.value = r.year;
          tdYear.appendChild(inpY);

          const tdSales = document.createElement('td');
          const inpS = document.createElement('input');
          inpS.type = 'number';
          inpS.min = '0';
          inpS.value = r.sales;
          tdSales.appendChild(inpS);

          const tdAct = document.createElement('td');
          const saveBtn = document.createElement('button');
          saveBtn.className = 'btn';
          saveBtn.textContent = 'Guardar';
          saveBtn.addEventListener('click', async () => {{
            await saveRow(r.id, inpY.value, inpS.value);
          }});

          const cancelBtn = document.createElement('button');
          cancelBtn.className = 'btn';
          cancelBtn.textContent = 'Cancelar';
          cancelBtn.addEventListener('click', load);

          tdAct.appendChild(saveBtn);
          tdAct.appendChild(cancelBtn);

          tr.appendChild(tdYear);
          tr.appendChild(tdSales);
          tr.appendChild(tdAct);
        }}

        async function saveRow(id, year, sales) {{
          const payload = {{
            year: year ? Number(year) : null,
            sales: sales ? Number(sales) : null
          }};
          const res = await fetch('/api/sales/' + id, {{
            method: 'PUT',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify(payload)
          }});
          const ans = await res.json().catch(() => ({{ success:false, message:'Error desconocido' }}));
          if (!res.ok || !ans.success) {{
            alert('No se pudo actualizar: ' + (ans.message || res.statusText));
            return;
          }}
          await load();
        }}

        async function delRow(id) {{
          if (!confirm('¿Eliminar este registro?')) return;
          const res = await fetch('/api/sales/' + id, {{ method: 'DELETE' }});
          const ans = await res.json().catch(() => ({{ success:false }}));
          if (!res.ok || !ans.success) {{
            alert('No se pudo eliminar.');
            return;
          }}
          await load();
        }}

        // Alta
        addForm.addEventListener('submit', async (e) => {{
          e.preventDefault();
          msg.textContent = 'Guardando…';

          const y = (document.getElementById('year').value || '').trim();
          const s = (document.getElementById('sales').value || '').trim();

          const payload = {{
            book_id: bookId,
            year: Number(y),
            sales: Number(s)
          }};

          try {{
            const res = await fetch('/api/sales', {{
              method: 'POST',
              headers: {{ 'Content-Type': 'application/json' }},
              body: JSON.stringify(payload)
            }});
            const ans = await res.json().catch(() => ({{ success:false, message:'Error desconocido' }}));
            if (!res.ok || !ans.success) {{
              msg.textContent = 'Error: ' + (ans.message || res.statusText);
              return;
            }}
            msg.textContent = 'Registro agregado ✔';
            addForm.reset();
            await load();
            setTimeout(() => {{ msg.textContent = ''; }}, 1500);
          }} catch (err) {{
            msg.textContent = 'Error: ' + err;
          }}
        }});

        // Carga inicial
        load().catch(err => {{
          tbody.innerHTML = '<tr><td colspan="3" class="danger">Error: ' + err + '</td></tr>';
        }});
      </script>
    "#);

    RawHtml(render_page("Ventas por año", &body))
}
