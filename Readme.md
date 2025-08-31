# Libro Reviews - Docker Setup

Aplicación web con Rust y SQLite, configurada para desarrollo con Docker.

## 🚀 Comandos para incio Rapido

### Prerrequisitos (revisar)
- Docker
- Docker Compose
- Cargo (preguntar)
- SQLite

### Pasos de Inicialización
```bash
# 1. Clonar el repositorio
git clone <tu-repo>
cd software_architecture_assigment1

# 2. Constuir aplicacion
docker-compose up --build

# 3. Levantar la aplicación
docker-compose up -d

# 4. Probar que funciona
curl http://localhost:8000/
```

## Explicacion de los comandos importantes

### 🐢 Comandos Lentos (solo en caso de ser nesesarios)

#### `docker-compose build`

- **Tiempo**: 10-15 minutos
- **Qué hace**: Construye la imagen de Docker desde cero
- **Cuándo usar**: Solo cuando cambies el Dockerfile o primera vez
- **Base de datos**: NO la afecta
- **Variacion para levantar APP**: `docker-compose up --build`
- **Variacion para limpieza profunda**: `docker-compose build --no-cache`

### ⚡ Comandos RÁPIDOS (desarrollo diario)

#### `docker-compose up -d`
- **Tiempo**: 2-3 segundos
- **Qué hace**: Levanta la aplicación en segundo plano
- **Cuándo usar**: Para iniciar la app después de `docker-compose down`
- **Base de datos**: NO la reinicia, mantiene todos los datos
- **Archivos**: Monta el código fuente como volumen (cambios se ven automáticamente)

#### `docker-compose restart app`
- **Tiempo**: 2-3 segundos ⭐ **MÁS IMPORTANTE**
- **Qué hace**: Reinicia solo la aplicación (NO reconstruye imagen)
- **Cuándo usar**: Cada vez que hagas cambios en el código (src/main.rs, etc.)
- **Base de datos**: NO la reinicia, mantiene todos los datos
- **Archivos**: Aplica cambios del código fuente inmediatamente

#### `docker-compose logs -f app`
- **Tiempo**: Instantáneo
- **Qué hace**: Muestra logs de la aplicación en tiempo real
- **Cuándo usar**: Para monitorear la app y ver errores

#### `docker-compose down`
- **Tiempo**: 1-2 segundos
- **Qué hace**: Detiene y elimina contenedores
- **Cuándo usar**: Para detener completamente la aplicación
- **Base de datos**: NO la elimina, los datos persisten en el archivo db.sqlite

#### `docker-compose ps`
- **Tiempo**: Instantáneo
- **Qué hace**: Muestra el estado de los servicios
- **Cuándo usar**: Para verificar si la app está corriendo




## 🐦📁 Base de datos (db.sqlite)

- NUNCA se reinicia con restart o up
- Datos persisten ente reinicicios
- Solo se pierden los datos si se elimina manualmente el archivo 





## 🛠️ Alternativa con Make (Segunda Opción) (preguntar si ya se elimino)

Si prefieres comandos más cortos, puedes usar Make:

```bash
# Equivalente a Docker Compose:
make up        # = docker-compose up -d
make restart   # = docker-compose restart app
make logs      # = docker-compose logs -f app
make down      # = docker-compose down
make build     # = docker-compose build
```

