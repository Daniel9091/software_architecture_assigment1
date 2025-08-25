#!/usr/bin/env bash
set -euo pipefail

if docker compose version &>/dev/null; then
  COMPOSE_CMD=(docker compose)
else
  COMPOSE_CMD=(docker-compose)
fi

echo "🔨 Build..."
"${COMPOSE_CMD[@]}" build

echo "🚀 Up..."
"${COMPOSE_CMD[@]}" up -d

echo "🌐 http://localhost:8000"
echo "📜 Logs: ${COMPOSE_CMD[*]} logs -f app"
