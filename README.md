# Pulso

Pulso is a lightweight, self-hosted event ingestion and real-time analytics platform. Send events from any application, get rolling-window metrics, and set threshold-based alerts — all scoped per tenant via API key.

A single Pulso instance supports multiple isolated tenants. It's domain-agnostic: e-commerce, SaaS, ridesharing, games — any event-driven workload.

Conceptually a self-hosted alternative to Mixpanel or PostHog, without vendor lock-in or data leaving your infrastructure.

## Prerequisites

- Rust (via [rustup](https://rustup.rs)) — `cargo --version` should work
- PostgreSQL running locally (or in Docker) and reachable
- Redis running locally (or in Docker) — `redis-cli ping` should return `PONG`
- `sqlx-cli` for running migrations:
  ```
  cargo install sqlx-cli --no-default-features --features postgres
  ```

## Setup

### 1. Database

Create a role and database:
```
psql -d postgres -c "CREATE USER pulso WITH PASSWORD 'pulso' SUPERUSER;"
psql -d postgres -c "CREATE DATABASE pulso OWNER pulso;"
```

### 2. Environment

In `backend/`, create a `.env` file:
```
DATABASE_URL=postgres://pulso:pulso@localhost:5432/pulso
REDIS_URL=redis://127.0.0.1:6379
```

### 3. Migrations

Migrations run automatically on server startup. To run them manually or to add new ones:
```bash
# from backend/
sqlx migrate run
sqlx migrate add <description>
```

## Running the backend

```bash
# from backend/
cargo run
```

Server listens on `http://localhost:3000`.

### Seed a test API key

For local testing without creating a tenant through the API:
```bash
cargo run --bin seed
```
Inserts `test-key-123` for `tenant-acme`.

## API

All endpoints except `/health` and `POST /tenants` require an `x-api-key` header.

### Tenants
| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/tenants` | — | Create a tenant, get back a raw API key (shown once) |

### Events
| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/events` | ✓ | Ingest an event |
| GET | `/events` | ✓ | List the last 100 events for this tenant |

### Metrics
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/metrics` | ✓ | Rolling window counts per event type (5m / 15m / 1h / 24h) |

### Alerts
| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/alert-rules` | ✓ | Create a threshold alert rule |
| GET | `/alert-rules` | ✓ | List active alert rules |
| DELETE | `/alert-rules/{id}` | ✓ | Soft-delete an alert rule |
| GET | `/alerts` | ✓ | Alert event history (last 100) |

### Health
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/health` | — | Service health check |

## Quick start (end-to-end)

### 1. Create a tenant
```bash
curl -X POST http://localhost:3000/tenants \
  -H 'Content-Type: application/json' \
  -d '{"tenant_name":"Acme Co"}'
```
Returns `{ "tenant_id": "...", "api_key": "...", "tenant_name": "Acme Co" }`. Save the `api_key` — it's shown once.

### 2. Send events
```bash
curl -X POST http://localhost:3000/events \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: <your_api_key>' \
  -d '{"event_type":"checkout"}'
```

### 3. Query rolling metrics
```bash
curl http://localhost:3000/metrics -H 'x-api-key: <your_api_key>'
```
Returns counts per event type across the last 5 minutes, 15 minutes, 1 hour, and 24 hours.

### 4. Create an alert rule
Fires when `checkout` count exceeds 2 in the last 5 minutes:
```bash
curl -X POST http://localhost:3000/alert-rules \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: <your_api_key>' \
  -d '{"event_type":"checkout","rule_condition":"above","threshold":2,"time_window":"5m"}'
```
`rule_condition` accepts `"above"` or `"below"`. `time_window` accepts `"5m"`, `"15m"`, `"1h"`, or `"24h"`.

### 5. Check alert history
The background worker polls every 10 seconds. After the threshold is crossed:
```bash
curl http://localhost:3000/alerts -H 'x-api-key: <your_api_key>'
```
Returns alert events with `triggered_at` and `resolved_at` (null while open). Alerts auto-resolve when the condition clears.

## How it works

- **Ingestion** — `POST /events` writes to Postgres, then spawns a background task to update a Redis sorted set. Ingestion latency is not blocked by Redis.
- **Rolling windows** — Redis sorted sets keyed by `metrics:{tenant_id}:{event_type}`. Each member is scored by its timestamp in ms. `ZCOUNT` over the relevant window gives an exact rolling count; entries older than 24 hours are pruned on each write.
- **Alerting** — A Tokio background task polls all active rules every 10 seconds. It's edge-triggered: fires once when the condition is first met, resolves automatically when it clears. Alert rules are soft-deleted to preserve history.
- **Multi-tenancy** — All reads and writes are scoped to the `tenant_id` resolved from the API key. One tenant cannot access another's data.

## Frontend

The frontend (`frontend/`) is a Vite + React + TypeScript scaffold — not yet wired to the backend.
```bash
cd frontend
npm install
npm run dev
```
