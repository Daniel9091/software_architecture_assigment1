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
        .btn { padding: .25rem .5rem; border: 1px solid #ccc; border-radius: .375rem; background: #f7f7f7; cursor: pointer; text-decoration:none; color:inherit; }
        .btn:hover { background: #eee; }
        .muted { color:#666; }
        #adder { margin: .75rem 0 1rem; }
        #addForm { display:none; margin-top:.5rem; }
        #addForm .row { margin:.35rem 0; }
        #addForm input[type='text'], #addForm input[type='date'], #addForm textarea { width: 100%; max-width: 420px; padding:.35rem; }
        #msg { margin-top:.25rem; }
      </style>

      <p class="loading">Cargando autores...</p>

      <div id="adder">
        <button id="toggleAdd" class="btn">➕ Agregar autor</button>
        <form id="addForm">
          <div class="row">
            <label for="a_name"><b>Nombre</b></label>
            <input id="a_name" type="text" required />
          </div>
          <div class="row">
            <label for="a_country"><b>País</b></label>
            <input id="a_country" type="text" />
          </div>
          <div class="row">
            <label for="a_birth"><b>Fecha de nacimiento</b></label>
            <input id="a_birth" type="date" />
            <div class="muted">Formato: YYYY-MM-DD</div>
          </div>
          <div class="row">
            <label for="a_desc"><b>Descripción</b></label>
            <textarea id="a_desc" rows="3"></textarea>
          </div>
          <button type="submit" class="btn">Guardar</button>
          <button type="button" id="cancelAdd" class="btn">Cancelar</button>
          <div id="msg" class="muted"></div>
        </form>
      </div>

      <ul id="list"></ul>

      <script>
        const loadingEl = document.querySelector('.loading');
        const list = document.getElementById('list');

        async function load() {
          try {
            const res = await fetch('/api/authors');
            if (!res.ok) throw new Error(res.statusText);
            const wrapper = await res.json();
            if (!wrapper || !wrapper.success) throw new Error((wrapper && wrapper.message) || 'Respuesta inválida');
            const data = wrapper.data || [];
            list.innerHTML = '';
            if (!Array.isArray(data) || data.length === 0) {
              list.innerHTML = '<li>No hay autores</li>';
              loadingEl.style.display = 'none';
              return;
            }
            for (const a of data) {
              const li = document.createElement('li');

              const name = document.createElement('span');
              name.textContent = a.name ?? '(sin nombre)';

              const btn = document.createElement('a');
              btn.className = 'btn';
              btn.textContent = 'Ver';
              btn.href = '/authors/' + a.id;

              li.appendChild(name);
              li.appendChild(btn);
              list.appendChild(li);
            }
            loadingEl.style.display = 'none';
          } catch (err) {
            loadingEl.textContent = 'Error cargando autores: ' + err;
          }
        }

        // Toggle del formulario de alta
        const toggleAdd = document.getElementById('toggleAdd');
        const addForm = document.getElementById('addForm');
        const cancelAdd = document.getElementById('cancelAdd');
        const msg = document.getElementById('msg');

        toggleAdd.addEventListener('click', () => {
          addForm.style.display = addForm.style.display === 'none' || addForm.style.display === '' ? 'block' : 'none';
          msg.textContent = '';
        });

        cancelAdd.addEventListener('click', () => {
          addForm.reset();
          addForm.style.display = 'none';
          msg.textContent = '';
        });

        // POST /api/authors
        addForm.addEventListener('submit', async (e) => {
          e.preventDefault();
          msg.textContent = 'Guardando…';

          const payload = {
            name: document.getElementById('a_name').value || null,
            country: document.getElementById('a_country').value || null,
            birth_date: document.getElementById('a_birth').value || null,
            description: document.getElementById('a_desc').value || null
          };

          try {
            const res = await fetch('/api/authors', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify(payload)
            });
            const ans = await res.json().catch(() => ({ success:false, message:'Error desconocido' }));
            if (!res.ok || !ans.success) {
              msg.textContent = 'Error: ' + (ans.message || res.statusText);
              return;
            }
            msg.textContent = 'Autor creado ✔';
            addForm.reset();
            addForm.style.display = 'none';
            await load(); // refresca la lista
          } catch (err) {
            msg.textContent = 'Error: ' + err;
          }
        });

        load();
      </script>
    "#;
    RawHtml(render_page("Autores", body))
}

#[get("/authors/<_id>")]
pub async fn authors_show(_id: i32, _pool: &State<Db>) -> RawHtml<String> {
    let body = r#"
      <style>
        form .row { margin: .4rem 0; }
        label { display:block; font-weight:600; margin-bottom:.2rem; }
        input[type="text"], input[type="date"], textarea { width: 100%; max-width: 480px; padding:.4rem; }
        .actions { margin-top: .8rem; display:flex; gap:.5rem; align-items:center; }
        .danger { color:#b00020; }
        .muted { color:#666; font-size:.9rem; }
        .hidden { display:none; }
      </style>

      <div id="view">
        <h2 id="name"></h2>
        <p><b>País:</b> <span id="country"></span></p>
        <p><b>Nacimiento:</b> <span id="birth"></span></p>
        <p><b>Descripción:</b><br><span id="desc"></span></p>

        <h3>Libros</h3>
        <ul id="books"><li class="muted">Cargando…</li></ul>

        <div class="actions">
          <button id="editBtn">Editar</button>
          <button id="deleteBtn" class="danger">Eliminar</button>
          <a href="/authors" class="muted">← Volver a autores</a>
        </div>
      </div>

      <!-- Formulario de edición -->
      <form id="editForm" class="hidden">
        <h2>Editar autor</h2>
        <div class="row">
          <label for="f_name">Nombre</label>
          <input id="f_name" type="text" required />
        </div>
        <div class="row">
          <label for="f_country">País</label>
          <input id="f_country" type="text" />
        </div>
        <div class="row">
          <label for="f_birth">Fecha de nacimiento</label>
          <input id="f_birth" type="date" />
          <div class="muted">Formato: YYYY-MM-DD</div>
        </div>
        <div class="row">
          <label for="f_desc">Descripción</label>
          <textarea id="f_desc" rows="4"></textarea>
        </div>
        <div class="actions">
          <button type="submit">Guardar</button>
          <button type="button" id="cancelBtn">Cancelar</button>
        </div>
        <p id="formMsg" class="muted"></p>
      </form>

      <script>
        // tomamos el id desde la URL: /authors/<id>
        const id = Number(location.pathname.split('/').filter(Boolean).pop());

        function fmt(d) {
          return d || '';
        }

        async function load() {
          const res = await fetch('/api/authors/' + id + '/details');
          if (!res.ok) throw new Error(res.statusText);
          const wrap = await res.json();
          if (!wrap.success) throw new Error(wrap.message || 'Error');
          const details = wrap.data;

          const a = details.author || {};
          document.getElementById('name').textContent = a.name || 'Sin nombre';
          document.getElementById('country').textContent = a.country || '—';
          document.getElementById('birth').textContent = fmt(a.birth_date);
          document.getElementById('desc').textContent = a.description || '—';

          // Prefill form
          document.getElementById('f_name').value = a.name || '';
          document.getElementById('f_country').value = a.country || '';
          document.getElementById('f_birth').value = a.birth_date || '';
          document.getElementById('f_desc').value = a.description || '';

          // Libros (sin template literals para evitar conflictos)
          const books = details.books || [];
          const list = document.getElementById('books');
          list.innerHTML = '';
          if (Array.isArray(books) && books.length) {
            for (const b of books) {
              const li = document.createElement('li');
              const y = (b.published_year ?? b.publication_date);
              const year = y ? ' (' + y + ')' : '';
              li.innerHTML = '<a href="/books/' + b.id + '">' + b.title + '</a>' + year;
              list.appendChild(li);
            }
          } else {
            list.innerHTML = '<li class="muted">Este autor aún no tiene libros.</li>';
          }
        }

        const editBtn = document.getElementById('editBtn');
        const deleteBtn = document.getElementById('deleteBtn');
        const form = document.getElementById('editForm');
        const view = document.getElementById('view');
        const cancelBtn = document.getElementById('cancelBtn');
        const formMsg = document.getElementById('formMsg');

        editBtn.addEventListener('click', () => {
          view.classList.add('hidden');
          form.classList.remove('hidden');
          formMsg.textContent = '';
        });

        cancelBtn.addEventListener('click', () => {
          form.classList.add('hidden');
          view.classList.remove('hidden');
        });

        form.addEventListener('submit', async (e) => {
          e.preventDefault();
          formMsg.textContent = 'Guardando…';

          const payload = {
            name: document.getElementById('f_name').value || null,
            country: document.getElementById('f_country').value || null,
            birth_date: document.getElementById('f_birth').value || null,
            description: document.getElementById('f_desc').value || null,
          };

          const res = await fetch('/api/authors/' + id, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload),
          });

          const ans = await res.json().catch(() => ({ success:false, message:'Error desconocido' }));
          if (!res.ok || !ans.success) {
            formMsg.textContent = 'Error al actualizar: ' + (ans.message || res.statusText);
            return;
          }
          await load();
          form.classList.add('hidden');
          view.classList.remove('hidden');
          formMsg.textContent = '';
        });

        deleteBtn.addEventListener('click', async () => {
          if (!confirm('¿Seguro que quieres eliminar este autor? Esta acción es irreversible.')) return;
          const res = await fetch('/api/authors/' + id, { method: 'DELETE' });
          const ans = await res.json().catch(() => ({ success:false }));
          if (!res.ok || !ans.success) {
            alert('No se pudo eliminar el autor.');
            return;
          }
          window.location.href = '/authors';
        });

        load().catch(err => {
          document.body.innerHTML = '<p>Error cargando autor: ' + err + '</p>';
        });
      </script>
    "#;

    RawHtml(render_page("Autor", body))
}