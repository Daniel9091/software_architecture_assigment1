#!/bin/bash

# Script de inicio rápido para Libro Reviews
# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Iniciando Libro Reviews con Docker...${NC}"

# Verificar si Docker está instalado
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker no está instalado. Por favor instala Docker primero.${NC}"
    echo "Visita: https://docs.docker.com/get-docker/"
    exit 1
fi

# Verificar si Docker Compose está disponible
if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}❌ Docker Compose no está disponible. Por favor instala Docker Compose.${NC}"
    exit 1
fi

# Verificar si Docker está ejecutándose
if ! docker info &> /dev/null; then
    echo -e "${RED}❌ Docker no está ejecutándose. Por favor inicia Docker.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker está funcionando correctamente${NC}"

# Construir la imagen si no existe
echo -e "${YELLOW}🔨 Construyendo imagen de Docker...${NC}"
docker-compose build

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Error al construir la imagen${NC}"
    exit 1
fi

# Levantar la aplicación
echo -e "${YELLOW}🚀 Levantando la aplicación...${NC}"
docker-compose up -d

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Error al levantar la aplicación${NC}"
    exit 1
fi

# Esperar un momento para que la aplicación se inicie
echo -e "${YELLOW}⏳ Esperando que la aplicación se inicie...${NC}"
sleep 5

# Verificar el estado
echo -e "${YELLOW}📊 Verificando estado de la aplicación...${NC}"
docker-compose ps

# Mostrar logs
echo -e "${GREEN}✅ Aplicación iniciada correctamente!${NC}"
echo -e "${BLUE}🌐 La aplicación está disponible en: http://localhost:8000${NC}"
echo ""
echo -e "${YELLOW}Comandos útiles:${NC}"
echo "  make logs      - Ver logs en tiempo real"
echo "  make down      - Detener la aplicación"
echo "  make restart   - Reiniciar la aplicación"
echo "  make help      - Ver todos los comandos disponibles"
echo ""
echo -e "${GREEN}¡Disfruta desarrollando! 🎉${NC}" 