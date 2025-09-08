# make_report.py
import json, sys, re

def parse_filename(fname):
    m = re.search(r'/([a-zA-Z]+)_(\d+)_\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2}\.json$', fname)
    if not m:
        return "unknown", ""
    return m.group(1), m.group(2)

def read(path):
    with open(path, 'r') as f:
        return json.load(f)

def get(metric, summary, key):
    return summary.get('metrics', {}).get(metric, {}).get(key)

def get_pct(metric, summary, pct):
    return summary.get('metrics', {}).get(metric, {}).get('percentiles', {}).get(pct)

def row(summary, label, total):
    http_reqs = int(get('http_reqs', summary, 'count') or 0)
    p95 = float(get_pct('http_req_duration', summary, 'p(95)') or 0.0)
    p99 = float(get_pct('http_req_duration', summary, 'p(99)') or 0.0)
    avg = float(get('http_req_duration', summary, 'avg') or 0.0)
    failed_rate = float(get('http_req_failed', summary, 'rate') or 0.0) * 100.0

    # Contadores por familia
    def mcount(name):
        return int(get(name, summary, 'count') or 0)

    c2xx = mcount('code_2xx')
    c3xx = mcount('code_3xx')
    c4xx = mcount('code_4xx')
    c5xx = mcount('code_5xx')

    # RPS aproximado asumiendo 300s por corrida (ajusta si cambias DURATION_S)
    rps = round(http_reqs / 300.0, 2)

    return {
        "label": label,
        "total": int(total) if total else "",
        "http_reqs": http_reqs,
        "p95_ms": round(p95, 2),
        "p99_ms": round(p99, 2),
        "avg_ms": round(avg, 2),
        "rps": rps,
        "error_rate_percent": round(failed_rate, 3),
        "2xx": c2xx,
        "3xx": c3xx,
        "4xx": c4xx,
        "5xx": c5xx,
    }

def main():
    files = sys.argv[1:]
    rows = []
    for f in files:
        label, total = parse_filename(f)
        s = read(f)
        rows.append(row(s, label, total))

    rows.sort(key=lambda r: (r["label"], r["total"]))
    print("label,total,http_reqs,p95_ms,p99_ms,avg_ms,rps,error_rate_percent,2xx,3xx,4xx,5xx")
    for r in rows:
        print(f'{r["label"]},{r["total"]},{r["http_reqs"]},{r["p95_ms"]},{r["p99_ms"]},{r["avg_ms"]},{r["rps"]},{r["error_rate_percent"]},{r["2xx"]},{r["3xx"]},{r["4xx"]},{r["5xx"]}')

if __name__ == "__main__":
    main()
