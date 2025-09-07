import http from 'k6/http';
import { sleep, check } from 'k6';

const TARGET = __ENV.TARGET || 'http://localhost';
const TOTAL = Math.max(1, parseInt(__ENV.TOTAL || '100', 10)); // 1|10|100|1000|5000
const PERIOD_MS = Math.floor(300000 / TOTAL); // 5 min = 300000 ms
const PERIOD_S = PERIOD_MS / 1000;

export const options = {
  scenarios: {
    paced_over_5m: {
      executor: 'per-vu-iterations',
      vus: 1,
      iterations: TOTAL,
      maxDuration: '6m',
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<2000'],
  },
};

export default function () {
  const res = http.get(`${TARGET}/`);
  check(res, { 'status 2xx/3xx': (r) => r.status >= 200 && r.status < 400 });
  sleep(PERIOD_S);
}