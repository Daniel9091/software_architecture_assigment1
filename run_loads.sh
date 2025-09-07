#!/usr/bin/env bash
set -euo pipefail
NET=software_architecture_assigment1_web
SCRIPTS="$PWD/loadtest-k6"

run_one () {
  TARGET="$1"   # http://traefik  o  http://app:8000
  LABEL="$2"    # traefik | direct
  TOTAL="$3"    # 1,10,100,1000,5000
  stamp=$(date +%Y%m%d-%H%M%S)
  base="results/${LABEL}-${TOTAL}-${stamp}"
  echo "==> ${LABEL} | TOTAL=${TOTAL} | TARGET=${TARGET}"

  # Métricas docker cada 5s
  (
    printf "time,container,name,cpu,mem,pids\n"
    while true; do
      ts=$(date +%s)
      docker stats --no-stream --format "{{.Container}},{{.Name}},{{.CPUPerc}},{{.MemUsage}},{{.PIDs}}" \
      | sed "s/^/${ts},/"
      sleep 5
    done
  ) > "${base}-docker-stats.csv" &
  mon_pid=$!

  # (opcional) hilos/procesos al inicio
  docker ps --format '{{.Names}}' | while read cname; do
    docker exec "$cname" sh -lc 'ps -o pid,comm,thcount --no-headers 2>/dev/null || true' \
      | sed "s/^/${cname},/" >> "${base}-threads-start.txt" || true
  done

  # Ejecutar k6 (5 min según tu script.js y TOTAL)
  docker run --rm -i --network "$NET" \
    -e TARGET="$TARGET" -e TOTAL="$TOTAL" \
    -v "$SCRIPTS:/scripts" \
    grafana/k6 run /scripts/script.js \
    --summary-export "${base}-k6.json" | tee "${base}-k6.log"

  # Parar métricas
  kill $mon_pid 2>/dev/null || true
  wait $mon_pid 2>/dev/null || true

  # (opcional) hilos/procesos al final
  docker ps --format '{{.Names}}' | while read cname; do
    docker exec "$cname" sh -lc 'ps -o pid,comm,thcount --no-headers 2>/dev/null || true' \
      | sed "s/^/${cname},/" >> "${base}-threads-end.txt" || true
  done

  echo "Guardado en ${base}-*"
}

# Ya corriste TOTAL=1 para ambos y TOTAL=50 para traefik,
# así que hacemos SOLO lo que falta del enunciado:
# 10, 100, 1000, 5000 para traefik y directo.

for TOTAL in 10 100 1000 5000; do
  run_one "http://traefik" "traefik" "$TOTAL"
done

for TOTAL in 10 100 1000 5000; do
  run_one "http://app:8000" "direct" "$TOTAL"
done
