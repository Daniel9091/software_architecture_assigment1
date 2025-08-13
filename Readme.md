# Libro Reviews

Una aplicación web para gestionar y revisar libros, construida con Rust y SQLite.

## 🚀 Instalación y Configuración


### Pasos de Instalación

1. **Clonar el repositorio**
   ```bash
   git clone git@github.com:Daniel9091/software_architecture_assigment1.git
   cd libro_reviews
   ```

2. **Instalar dependencias**
   ```bash
   cargo build
   ```

3. **Configurar la base de datos**
   ```bash
   # Instalar SQLx CLI
   cargo install sqlx-cli --features sqlite
   
   # Crear la base de datos
   sqlx database create
   ```

## 🏃‍♂️ Ejecutar la Aplicación

```bash
cargo run
```

La aplicación estará disponible en: **http://localhost:8000**
