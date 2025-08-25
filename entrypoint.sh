#!/usr/bin/env bash
set -euo pipefail

DB_PATH="/data/db.sqlite"
MIG_DIR="/app/migrations"

echo "[entrypoint] usando DB: $DB_PATH"

# Asegura PATH con cargo
export PATH="/usr/local/cargo/bin:/usr/local/rustup/toolchains/*/bin:$PATH"

# Crea DB si no existe y aplica migraciones
if [ ! -f "$DB_PATH" ]; then
  echo "[entrypoint] creando DB y aplicando migraciones..."
  mkdir -p "$(dirname "$DB_PATH")"
  sqlite3 "$DB_PATH" "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;"
  shopt -s nullglob
  for f in "$MIG_DIR"/*.sql; do
    echo "[entrypoint] aplicando $f"
    sqlite3 "$DB_PATH" < "$f"
  done
  echo "[entrypoint] ✅ migraciones listas"
else
  echo "[entrypoint] DB ya existe; saltando inicialización."
fi

echo "[entrypoint] iniciando app..."

# Opción A: con hot-reload (cargo-watch)
exec /usr/local/cargo/bin/cargo watch -x 'run --release'

# Opción B (sin hot-reload): comenta la línea de arriba y descomenta esta:
# exec /usr/local/cargo/bin/cargo run --release
