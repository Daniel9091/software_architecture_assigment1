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

#### Borrar volumenes:
- `docker volume rm {NOMBRE  BBDD}`
- `docker-compose down -v`

## Caché (Redis - BB8)
### Ventajas de ocupar Cache:
- Reducion tiempo de respuesta
- Dsminucion de carga para BBDD
### Ventajas y Desventajas de Redis
- **Ventajas**: Mas documentacion y robustez
- **Desventajas**: Mayor consumo de memoria y ligeramente mas compleja

### Caracteristicas Importantes:
- **BB8**: Se encarga de las consecciones de pulling de Redis
- **Cache.rs**: Archivo encargado de gestionar caché
- **Constantes**: Los nombres de las llaves en el caché y el tiempo TTL se manejan como CTE en Cache.rs
- **TTL**: Se considero tiempo prudente 5 minutos en el caché
- **Rutas Implementadas**: Solo se consideran las rutas GET para el cahce ya que el tiempo TTL es bajo, se añade demasiada complejidad inesesariamente si se implementa el borrado del cahce con las otras rutas (Fue comberzado explisitamente con el profesor)

### Visualisacion del Archvio de Caché


```bash
# 1. Ingreso al Caché (Abra otra terminal y ingrese comando)
docker exec -it software_architecture_assigment1-redis-1 redis-cli

# 2. Revision de keys guardadas en el Caché
KEYS *
KEYS {KEY}:*    # (Mas Espesifico)
KEYS *:{KEY}    # (Mas Espesifico)

# 3. Ver una clave espesifica
GET {KEY}

# 4. Ver tipo de dato
TYPE {KEY}

# Ver que le queda a la clave en el caché
TTL {KEY}
```


