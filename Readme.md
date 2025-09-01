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

- Datos persisten ente reinicicios
- Solo se pierden los datos si se elimina manualmente el archivo

### Manejo de BBDD con Doker

#### Volumenes del Proyecto:
`docker volume ls | grep software_architecture_assigment1`
- `docker volume inspect software_architecture_assigment1_dbdata`
- `docker volume inspect software_architecture_assigment1_redisdata`
- `docker volume inspect software_architecture_assigment1_rust_target`

#### Borrar vlolumens:
- `docker volume rm {NOMBRE  BBDD}`
- `docker-compose down -v`


## Caché (Redis - BB8)
- Reducion tiempo de respuesta
- Dsiminucion de carga para BBDD
- **Ventajas de Redis**: por mas documentacion y robustez
- **Desventajas de Redis**: Mayor consumo de memoria y ligeramente mas compleja

- **BB8**: Se encarga de las consecciones de pulling
- **Cache.rs**: Archivo encargado de gestionar caché
- caché de 5 minutos para casi todas las rutas API

### Visualisacion del Archvio de Caché
En otra terminal ingrese este comando para conecarse al cache

`docker exec -it software_architecture_assigment1-redis-1 redis-cli`

Para ver las claves que contiene en la terminal emergente ejecute

`KEYS *`

Para ver una clave espesifica

`KEYS {CLAVE}:*`

Para ver el contenido espesifico de la clave

`GET "{CLAVE}:list"`

Para ver el tipo de dato y el tiempo asignado por clave

`TYPE "{CLAVE}:list"`
`TTL "{CLAVE}:list"`



