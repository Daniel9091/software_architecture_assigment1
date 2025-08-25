FROM rust:1.82-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libsqlite3-dev sqlite3 pkg-config ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Manifiestos y código
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# (opcional) cachear deps
RUN cargo fetch || true

# Resto
COPY migrations ./migrations
COPY Rocket.toml ./Rocket.toml
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x /app/entrypoint.sh
RUN mkdir -p /data

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

# Hot-reload en dev
RUN cargo install cargo-watch

# Usa el entrypoint para inicializar DB y arrancar
ENTRYPOINT ["/app/entrypoint.sh"]
