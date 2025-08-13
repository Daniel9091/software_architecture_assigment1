# Libro Reviews - Docker Setup

Aplicación web con Rust y SQLite, configurada para desarrollo con Docker.

## 🚀 Inicio Rápido

### Prerrequisitos
- Docker
- Docker Compose

### Pasos de Inicialización
```bash
# 1. Clonar el repositorio
git clone <tu-repo>
cd software_architecture_assigment1

# 2. Levantar la aplicación
docker-compose up -d

# 3. Probar que funciona
curl http://localhost:8000/
# Debe responder: "Hola, Rocket + SQLite!"
```

## ⚡ Comandos de Docker - Explicación Detallada

### 🐌 Comandos LENTOS (solo cuando sea necesario)

#### `docker-compose build`
- **Tiempo**: 10-15 minutos
- **Qué hace**: Construye la imagen de Docker desde cero
- **Cuándo usar**: Solo cuando cambies el Dockerfile o primera vez
- **Base de datos**: NO la afecta

#### `docker-compose up --build`
- **Tiempo**: 10-15 minutos
- **Qué hace**: Construye imagen + levanta contenedor
- **Cuándo usar**: Solo la primera vez que configures el proyecto
- **Base de datos**: NO la afecta

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

## 🔄 Flujo de Desarrollo

### Configuración Inicial (UNA SOLA VEZ)
```bash
docker-compose build          # 10-15 minutos
docker-compose up -d          # 2-3 segundos
```

### Desarrollo Diario
```bash
# 1. Hacer cambios en src/main.rs
# 2. Guardar archivo
# 3. Reiniciar app (NO reconstruir)
docker-compose restart app    # ⚡ 2-3 segundos

# 4. Ver logs para confirmar cambios
docker-compose logs -f app
```

### Para Detener
```bash
docker-compose down           # 1-2 segundos
```

## 📁 ¿Qué Archivos/Carpetas Afecta Cada Comando?

### **Código Fuente (src/):**
- **Cambios automáticos**: Los archivos .rs se montan como volumen
- **Comando para aplicar**: `docker-compose restart app` (2-3 seg)

### **Base de Datos (db.sqlite):**
- **NUNCA se reinicia** con restart o up
- **Datos persisten** entre reinicios
- **Solo se pierde** si eliminas manualmente el archivo

### **Configuración (Dockerfile, docker-compose.yml):**
- **Requiere**: `docker-compose build` (10-15 min)
- **NO se aplica** con restart

## 🎯 Comandos Esenciales para Memorizar

```bash
# Desarrollo diario (RÁPIDOS):
docker-compose up -d          # Levantar
docker-compose restart app    # Reiniciar (para cambios de código) ⭐
docker-compose logs -f app    # Ver logs
docker-compose down           # Detener

# Solo cuando sea necesario (LENTOS):
docker-compose build          # Reconstruir imagen
```

## 🛠️ Alternativa con Make (Segunda Opción)

Si prefieres comandos más cortos, puedes usar Make:

```bash
# Equivalente a Docker Compose:
make up        # = docker-compose up -d
make restart   # = docker-compose restart app
make logs      # = docker-compose logs -f app
make down      # = docker-compose down
make build     # = docker-compose build
```

## 📝 Resumen Final

- **Primera vez**: `docker-compose build` (10-15 min) + `docker-compose up -d`
- **Cambios de código**: `docker-compose restart app` (2-3 seg)
- **Base de datos**: NUNCA se reinicia, datos persisten
- **Ver logs**: `docker-compose logs -f app`
- **Detener**: `docker-compose down`

**¡Nunca uses `docker-compose build` para cambios de código! Solo `restart app`** ⚡
