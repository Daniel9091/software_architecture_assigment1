#!/usr/bin/env bash
set -euo pipefail

SCRIPT="loadtest-k6/script.js"
OUTDIR="results"
mkdir -p "$OUTDIR/json" "$OUTDIR/csv" "$OUTDIR/metrics"

DURATION_S="${DURATION_S:-300}"  # 5 min por defecto
LOADS=(1 10 100 1000 5000)

# ===== Resolver k6 local o Docker =====
if command -v k6 >/dev/null 2>&1; then
  echo "[k6] usando binario local" >&2
  K6_RUNNER() {
    local TARGET="$1" TOTAL="$2" JSON_OUT="$3"
    TARGET="$TARGET" TOTAL="$TOTAL" DURATION_S="$DURATION_S" \
      k6 run --quiet --summary-export="$JSON_OUT" "$SCRIPT" 1>&2
  }
else
  echo "[k6] no encontrado, usando Docker (grafana/k6:latest)" >&2
  K6_RUNNER() {
    local TARGET="$1" TOTAL="$2" JSON_OUT="$3"

    # OJO: en Docker, 'localhost' dentro del contenedor NO es tu host.
    # - direct → host.docker.internal:8000
    # - traefik → resolvemos app.localhost al host
    docker run --rm -i \
      -v "$PWD":/work -w /work \
      --add-host=host.docker.internal:host-gateway \
      --add-host=app.localhost:host.docker.internal \
      grafana/k6:latest run \
      -e TARGET="$TARGET" -e TOTAL="$TOTAL" -e DURATION_S="$DURATION_S" \
      --quiet --summary-export "$JSON_OUT" "$SCRIPT" 1>&2
  }
fi

start_sampler() {
  local label="$1" total="$2" metrics="$3"
  local filter="${NAME_FILTER:-software_architecture_assigment1-}"
  ./metrics_sampler.sh "$label" "$total" "$metrics" "$filter" &
  echo $!
}

stop_sampler() {
  local pid="$1"
  kill "$pid" >/dev/null 2>&1 || true
  wait "$pid" 2>/dev/null || true
}

run_case () {
  local label="$1" target="$2" total="$3"

  local stamp="$(date +%Y-%m-%d-%H-%M-%S)"
  local json="$OUTDIR/json/${label}_${total}_${stamp}.json"
  local metrics="$OUTDIR/metrics/${label}_${total}_${stamp}.csv"

  echo
  echo "==> $label | TOTAL=$total | DUR=${DURATION_S}s | $(date)" >&2

  local sampler_pid
  sampler_pid="$(start_sampler "$label" "$total" "$metrics")"

  # Ejecutar k6 (todo su output se va a stderr para no romper la captura)
  if K6_RUNNER "$target" "$total" "$json"; then :; else
    echo "[WARN] k6 devolvió código != 0; continúo" >&2
  fi

  stop_sampler "$sampler_pid"

  echo "   k6:    $json" >&2
  echo "   stats: $metrics" >&2

  # SOLO imprimimos el path JSON por stdout (limpio)
  printf "%s\n" "$json"
}

JSONS=()

echo "### A) DIRECT (sin Traefik) ###" >&2
# Si usamos Docker k6, el target directo debe apuntar a host.docker.internal
if command -v k6 >/dev/null 2>&1; then
  DIRECT_TARGET="${DIRECT_TARGET:-http://localhost:8000}"
else
  DIRECT_TARGET="${DIRECT_TARGET:-http://host.docker.internal:8000}"
fi
for n in "${LOADS[@]}"; do
  JSONS+=("$(run_case direct "$DIRECT_TARGET" "$n")")
done

echo "### B) TRAEFIK (con proxy) ###" >&2
TRAEFIK_TARGET="${TRAEFIK_TARGET:-http://app.localhost}"
for n in "${LOADS[@]}"; do
  JSONS+=("$(run_case traefik "$TRAEFIK_TARGET" "$n")")
done

# Generar CSV resumen
python3 make_report.py "${JSONS[@]}" > "$OUTDIR/csv/summary.csv"

echo
echo "✅ Resumen k6:   $OUTDIR/csv/summary.csv" >&2
echo "✅ Métricas CPU/Mem/Threads (1 archivo por corrida) en: $OUTDIR/metrics/" >&2
