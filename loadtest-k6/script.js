/* eslint-env k6 */
import http from 'k6/http';
import { check } from 'k6';
import { Counter } from 'k6/metrics';

// Acceso defensivo a variables de entorno (evita __ENV null en init)
const ENV = (typeof __ENV === 'object' && __ENV !== null) ? __ENV : {};

// Config desde ENV (con defaults)
const TARGET = String(ENV.TARGET || 'http://app.localhost');
const TOTAL = Number.parseInt(ENV.TOTAL || '100', 10);
const DURATION_S = Number.parseInt(ENV.DURATION_S || '300', 10);
const RATE = Math.max(1, Math.ceil(TOTAL / DURATION_S));

// Contadores por familia de códigos
const code2xx = new Counter('code_2xx');
const code3xx = new Counter('code_3xx');
const code4xx = new Counter('code_4xx');
const code5xx = new Counter('code_5xx');

export const options = {
  scenarios: {
    carr: {
      executor: 'constant-arrival-rate',
      rate: RATE,
      timeUnit: '1s',
      duration: `${DURATION_S}s`,
      preAllocatedVUs: Math.min(400, Math.max(10, RATE * 3)),
      maxVUs: 2000,
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<2000', 'p(99)<5000'],
  },
};

export default function () {
  const res = http.get(`${TARGET}/?rnd=${Math.random()}`, {
    headers: { 'Cache-Control': 'no-cache' },
  });

  check(res, { '2xx/3xx': (r) => r.status >= 200 && r.status < 400 });

  if (res.status >= 200 && res.status < 300) code2xx.add(1);
  else if (res.status >= 300 && res.status < 400) code3xx.add(1);
  else if (res.status >= 400 && res.status < 500) code4xx.add(1);
  else if (res.status >= 500) code5xx.add(1);
}
