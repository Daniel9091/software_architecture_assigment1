#!/usr/bin/env python3
"""
Lee los archivos de results/ y genera:
- Gráficos PNG de p95 vs total y de http_reqs vs total (para traefik y direct)
- Gráficos PNG de CPU y PIDs promedio por contenedor conocido (app, traefik, redis) por escenario
- Un reporte Markdown con las tablas/resúmenes y links a los PNG

Uso:
  python3 make_report.py results
"""

import csv, re, sys, textwrap, pathlib
from collections import defaultdict, OrderedDict

try:
    import matplotlib.pyplot as plt
except ModuleNotFoundError:
    sys.exit(
        "Falta matplotlib. Instálalo y reintenta:\n\n"
        "   python3 -m pip install matplotlib\n"
    )

# -------------------- util --------------------
def ensure_dir(p: pathlib.Path):
    p.mkdir(parents=True, exist_ok=True)
    return p

def read_summary_k6(path: pathlib.Path):
    rows = []
    with path.open(newline="") as f:
        for i, r in enumerate(csv.DictReader(f)):
            # Espera: label,total,http_reqs,p95_ms
            try:
                rows.append({
                    "label": r["label"].strip(),
                    "total": int(r["total"]),
                    "http_reqs": int(r["http_reqs"]),
                    "p95_ms": float(r["p95_ms"])
                })
            except Exception:
                # Si hay filas rotas, las saltamos
                continue
    # Ordenado para graficar bonito
    rows.sort(key=lambda x: (x["label"], x["total"]))
    return rows

def parse_metric_blocks(txt: str, metric_key: str):
    """
    Parsea summary_*.txt de la forma:
      == <archivo.csv>
      <nombre> avg_<metric_key>=<valor><sufijo> samples=N

    Devuelve dict: escenario -> list[(container, valor)]
    """
    data = OrderedDict()
    blocks = re.split(r"\n==\s+", "\n" + txt.strip())
    for b in blocks:
        b = b.strip()
        if not b:
            continue
        first, *rest = b.splitlines()
        escenario = first.strip()
        rows = []
        for line in rest:
            m = re.search(
                rf"^(.*)\s+avg_{re.escape(metric_key)}=([0-9.]+)(?:[A-Za-z%]*)\s+samples=\d+$",
                line.strip()
            )
            if m:
                cname = m.group(1).strip()
                val = float(m.group(2))
                rows.append((cname, val))
        if rows:
            data[escenario] = rows
    return data

def pick_known_containers(rows):
    """
    De una lista [(name, val)] devuelve dict con claves: app, traefik, redis si aparecen
    (promedia por clave si hay varios matches).
    """
    buckets = {"app": [], "traefik": [], "redis": []}
    for name, val in rows:
        low = name.lower()
        if "traefik" in low:
            buckets["traefik"].append(val)
        if "app-1" in low or "-app" in low:
            buckets["app"].append(val)
        if "redis" in low:
            buckets["redis"].append(val)
    out = {}
    for k, vals in buckets.items():
        if vals:
            out[k] = sum(vals) / len(vals)
    return out

def escenario_tag(filename_csv: str):
    # ejemplo: "traefik-1000-20250907-120810-docker-stats.csv" -> "traefik-1000-20250907-120810"
    return filename_csv.replace("-docker-stats.csv", "")

def escenario_label_total(tag: str):
    # "traefik-1000-2025..." -> ("traefik", 1000)
    parts = tag.split("-")
    if len(parts) >= 2 and parts[1].isdigit():
        return parts[0], int(parts[1])
    return parts[0], None

# -------------------- main --------------------
def main(results_dir: pathlib.Path):
    out_dir = ensure_dir(results_dir / "report")
    figs_dir = ensure_dir(out_dir / "figs")

    # 1) Cargar k6
    k6_path = results_dir / "summary_k6.csv"
    if not k6_path.exists():
        sys.exit(f"No se encontró {k6_path}. Asegúrate de haber creado el resumen.")

    k6 = read_summary_k6(k6_path)

    # --- Gráfico p95 vs total ---
    for label in ["direct", "traefik"]:
        xs = [r["total"] for r in k6 if r["label"] == label]
        ys = [r["p95_ms"] for r in k6 if r["label"] == label]
        if not xs:
            continue
        plt.figure()
        plt.plot(xs, ys, marker="o")
        plt.xlabel("TOTAL requests (5 min)")
        plt.ylabel("p95 (ms)")
        plt.title(f"p95 de latencia vs total — {label}")
        plt.grid(True, which="both", linestyle="--", linewidth=0.5)
        p = figs_dir / f"p95_vs_total_{label}.png"
        plt.tight_layout()
        plt.savefig(p, dpi=160)
        plt.close()

    # --- Gráfico http_reqs vs total (debería ser ~2*TOTAL por tu script) ---
    for label in ["direct", "traefik"]:
        xs = [r["total"] for r in k6 if r["label"] == label]
        ys = [r["http_reqs"] for r in k6 if r["label"] == label]
        if not xs:
            continue
        plt.figure()
        plt.bar([str(x) for x in xs], ys)
        plt.xlabel("TOTAL (objetivo en script)")
        plt.ylabel("http_reqs observados")
        plt.title(f"Requests observados vs TOTAL — {label}")
        p = figs_dir / f"http_reqs_vs_total_{label}.png"
        plt.tight_layout()
        plt.savefig(p, dpi=160)
        plt.close()

    # 2) CPU y PIDs por contenedor (a partir de summaries txt)
    cpu_txt = (results_dir / "summary_cpu.txt").read_text(encoding="utf-8")
    pids_txt = (results_dir / "summary_pids.txt").read_text(encoding="utf-8")

    cpu_blocks = parse_metric_blocks(cpu_txt, "cpu")
    pids_blocks = parse_metric_blocks(pids_txt, "pids")

    # Estructuras: escenario_tag -> {app: v, traefik: v, redis: v}
    cpu_by_scenario = {}
    pids_by_scenario = {}

    for scen, rows in cpu_blocks.items():
        cpu_by_scenario[escenario_tag(scen)] = pick_known_containers(rows)
    for scen, rows in pids_blocks.items():
        pids_by_scenario[escenario_tag(scen)] = pick_known_containers(rows)

    # Orden de escenarios (por label y total)
    order_tags = []
    for label in ["direct", "traefik"]:
        for total in [10, 100, 1000, 5000]:
            # Busca el tag que empiece con label-total
            candidates = [t for t in cpu_by_scenario.keys() if t.startswith(f"{label}-{total}-")]
            if candidates:
                order_tags.append(candidates[0])

    def plot_grouped(data_by_scenario, metric_name, file_stub):
        # data_by_scenario: tag -> {app,traefik,redis}
        cats = ["app", "traefik", "redis"]
        xs = []
        series = {c: [] for c in cats}
        for tag in order_tags:
            xs.append(tag)
            vals = data_by_scenario.get(tag, {})
            for c in cats:
                series[c].append(vals.get(c, 0.0))

        plt.figure()
        import numpy as np
        X = np.arange(len(xs))
        width = 0.25
        for i, c in enumerate(cats):
            plt.bar(X + i*width - width, series[c], width, label=c)
        plt.xticks(X, xs, rotation=90)
        plt.ylabel(metric_name)
        plt.title(f"{metric_name} promedio por contenedor y escenario")
        plt.legend()
        plt.tight_layout()
        p = figs_dir / f"{file_stub}.png"
        plt.savefig(p, dpi=160)
        plt.close()

    # Gráficos agrupados
    plot_grouped(cpu_by_scenario, "CPU (%)", "cpu_by_container_and_scenario")
    plot_grouped(pids_by_scenario, "PIDs (prom)", "pids_by_container_and_scenario")

    # 3) Escribir un README/Reporte simple
    md = []
    md.append("# Reporte de cargas y métricas\n")
    md.append("## Resumen k6 (p95 y requests)\n")
    md.append("")
    # tabla markdown
    md.append("| label | total | http_reqs | p95_ms |")
    md.append("|---|---:|---:|---:|")
    for r in k6:
        md.append(f"| {r['label']} | {r['total']} | {r['http_reqs']} | {r['p95_ms']:.2f} |")

    md.append("\n### Gráficos de latencia y volumen\n")
    for label in ["direct", "traefik"]:
        a = f"figs/p95_vs_total_{label}.png"
        b = f"figs/http_reqs_vs_total_{label}.png"
        if (figs_dir / f"p95_vs_total_{label}.png").exists():
            md.append(f"![p95 vs total — {label}]({a})")
        if (figs_dir / f"http_reqs_vs_total_{label}.png").exists():
            md.append(f"![reqs vs total — {label}]({b})")

    md.append("\n## CPU y PIDs por contenedor conocido (app / traefik / redis)\n")
    if (figs_dir / "cpu_by_container_and_scenario.png").exists():
        md.append("![CPU por contenedor y escenario](figs/cpu_by_container_and_scenario.png)")
    if (figs_dir / "pids_by_container_and_scenario.png").exists():
        md.append("![PIDs por contenedor y escenario](figs/pids_by_container_and_scenario.png)")

    (out_dir / "REPORT.md").write_text("\n".join(md) + "\n", encoding="utf-8")

    print(f"Listo ✅")
    print(f"- Carpeta de figuras: {figs_dir}")
    print(f"- Reporte Markdown: {out_dir/'REPORT.md'}")
    print("\nPuedes previsualizar el REPORT.md en VS Code o convertirlo a PDF si lo prefieres.")
    print("Ejemplo para PDF (opcional):")
    print("  pandoc results/report/REPORT.md -o results/report/REPORT.pdf")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Uso: python3 make_report.py results")
        sys.exit(1)
    main(pathlib.Path(sys.argv[1]).resolve())