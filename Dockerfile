# Imagen simple de Rust con SQLite
FROM rust:latest

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Crear directorio de trabajo
WORKDIR /app

# Copiar archivos del proyecto
COPY . .

# Construir la aplicación
RUN cargo build --release

# Exponer puerto
EXPOSE 8000

# Comando por defecto
CMD ["cargo", "run", "--release"] 