.PHONY: help build up down logs clean restart shell

# Comando por defecto
help: ## Mostrar esta ayuda
	@echo "Comandos disponibles:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Construir la imagen de Docker
	docker-compose build

up: ## Levantar la aplicación
	docker-compose up -d

down: ## Detener la aplicación
	docker-compose down

logs: ## Ver logs de la aplicación
	docker-compose logs -f app

clean: ## Limpiar contenedores e imágenes
	docker-compose down --rmi all --volumes --remove-orphans
	docker system prune -f

restart: ## Reiniciar la aplicación
	docker-compose restart app

shell: ## Acceder al shell del contenedor
	docker-compose exec app /bin/bash

status: ## Ver estado de los servicios
	docker-compose ps

# Comando para desarrollo rápido
dev: build up logs ## Construir, levantar y mostrar logs (desarrollo) 