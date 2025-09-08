#!/usr/bin/env bash
set -euo pipefail

# Uso: ./metrics_sampler.sh <label> <total> <outfile.csv> [NAME_FILTER]
LABEL="${1:-run}"
TOTAL="${2:-0}"
OUT="${3:-metrics.csv}"
NAME_FILTER="${4:-}"     # opcional, p.ej. "software_architecture_assigment1-"
INTERVAL="${INTERVAL:-2}"  # segundos

echo "timestamp,label,total,container,cpu_percent,mem_mb,threads" > "$OUT"

while true; do
  TS="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"

  # Lista de contenedores (ID y nombre)
  if [ -n "$NAME_FILTER" ]; then
    CONTAINERS=$(docker ps --format '{{.ID}} {{.Names}}' | grep "$NAME_FILTER" || true)
  else
    CONTAINERS=$(docker ps --format '{{.ID}} {{.Names}}')
  fi

  # Si no hay contenedores, duerme y sigue
  if [ -z "$CONTAINERS" ]; then
    sleep "$INTERVAL"
    continue
  fi

  # Recorre contenedor por contenedor
  echo "$CONTAINERS" | while read -r CID CNAME; do
    # CPU/Mem de este contenedor
    # Ejemplo de docker stats format: "123abc name 5.23% 12.3MiB / ..."
    STATS_LINE="$(docker stats --no-stream --format '{{.ID}} {{.Name}} {{.CPUPerc}} {{.MemUsage}}' --filter id="$CID" 2>/dev/null | sed 's|/.*||')"
    # Si está vacío, pasar
    [ -z "$STATS_LINE" ] && continue

    CPU_PCT="$(echo "$STATS_LINE" | awk '{print $3}' | tr -d '%')"
    RAW_MEM="$(echo "$STATS_LINE" | awk '{print $4}')"
    UNIT="$(echo "$STATS_LINE" | awk '{print $5}')"

    # Normalizar a MB
    case "$UNIT" in
      MiB) MEM_MB="$RAW_MEM" ;;
      GiB) MEM_MB="$(awk -v g="$RAW_MEM" 'BEGIN{printf "%.2f", g*1024}')" ;;
      KiB) MEM_MB="$(awk -v k="$RAW_MEM" 'BEGIN{printf "%.2f", k/1024}')" ;;
      B)   MEM_MB="$(awk -v b="$RAW_MEM" 'BEGIN{printf "%.4f", b/1024/1024}')" ;;
      *)   MEM_MB="$RAW_MEM" ;;
    esac

    # Total de hilos dentro del contenedor
    THREADS="$(docker exec "$CID" sh -c "ps -e -o nlwp --no-headers 2>/dev/null | awk '{s+=\$1} END{print s+0}'" 2>/dev/null || echo 0)"

    echo "$TS,$LABEL,$TOTAL,$CNAME,$CPU_PCT,$MEM_MB,$THREADS" >> "$OUT"
  done

  sleep "$INTERVAL"
done
