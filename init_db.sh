#!/bin/bash

echo "🚀 Inicializando base de datos SQLite..."

# Verificar si sqlx-cli está instalado
if ! command -v sqlx &> /dev/null; then
    echo "❌ sqlx-cli no está instalado. Instalando..."
    cargo install sqlx-cli --features sqlite
fi

# Crear directorio de migraciones si no existe
mkdir -p migrations

# Crear la base de datos si no existe
sqlx database create

# Ejecutar migraciones
echo "📊 Aplicando esquema de base de datos..."
sqlx migrate run

echo "✅ Base de datos inicializada correctamente!"
echo "📚 Datos de ejemplo cargados:"
echo "   - 5 autores famosos"
echo "   - 5 libros clásicos"
echo "   - 6 reseñas de ejemplo"
echo "   - 20 registros de ventas anuales"
echo ""
echo "🌐 Para ejecutar la aplicación:"
echo "   cargo run"
echo ""
echo "🔗 La aplicación estará disponible en: http://localhost:8000" 