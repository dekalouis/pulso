# Pulso

Pulso is a lightweight, self-hosted event ingestion and real-time analytics platform. Send events from any application, get rolling-window metrics, and set threshold-based alerts - all scoped per tenant via API key.

A single Pulso instance supports multiple isolated tenants. It's domain-agnostic: e-commerce, SaaS, ridesharing, games - any event-driven workload.

## Motivation

Product analytics tools like Mixpanel and PostHog are great, but they mean vendor lock-in and sending your data to someone else's infrastructure. For many event-driven workloads, what you actually need is small: ingest events, see live rolling counts, and get told when something crosses a threshold.

Pulso is that, self-hosted:

- **Your data stays yours** - everything lives in your own Postgres and Redis.
- **Multi-tenant by design** - one instance serves many isolated tenants, each with its own API key.
- **Fast ingestion, exact windows** - events are persisted to Postgres, while Redis sorted sets give exact 5m / 15m / 1h / 24h rolling counts without blocking writes.
- **Simple alerting** - edge-triggered threshold rules that fire once and auto-resolve.

It is also a portfolio project exploring a Rust / Axum / sqlx backend with a React / TypeScript frontend.

## Quick Start

### Prerequisites

- Rust (via [rustup](https://rustup.rs)) - `cargo --version` should work
- PostgreSQL, running and reachable
- Redis, running - `redis-cli ping` should return `PONG`
- Node.js and npm (for the dashboard)

### 1. Database

```bash
psql -d postgres -c "CREATE USER pulso WITH PASSWORD 'pulso' SUPERUSER;"
psql -d postgres -c "CREATE DATABASE pulso OWNER pulso;"
```

### 2. Environment

In `backend/`, create a `.env` file (see `backend/.env.example`):

```
DATABASE_URL=postgres://pulso:pulso@localhost:5432/pulso
REDIS_URL=redis://127.0.0.1:6379
```

### 3. Run the backend

```bash
cd backend
cargo run
```

The server listens on `http://localhost:3000`. Migrations run automatically on startup.

### 4. Run the dashboard

```bash
cd frontend
npm install
npm run dev
```

Open the URL Vite prints (usually `http://localhost:5173`) and log in with a tenant API key (see below). The dashboard expects the backend at `http://localhost:3000`.

## Usage

### Create a tenant

```bash
curl -X POST http://localhost:3000/tenants \
  -H 'Content-Type: application/json' \
  -d '{"tenant_name":"Acme Co"}'
```

Returns `{ "tenant_id": "...", "api_key": "...", "tenant_name": "Acme Co" }`. Save the `api_key` - it is shown once.

For local testing without creating a tenant, `cargo run --bin seed` (from `backend/`) inserts `test-key-123` for `tenant-acme`.

### Send events

```bash
curl -X POST http://localhost:3000/events \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: <your_api_key>' \
  -d '{"event_type":"checkout"}'
```

### Query rolling metrics

```bash
curl http://localhost:3000/metrics -H 'x-api-key: <your_api_key>'
```

Returns counts per event type across the last 5 minutes, 15 minutes, 1 hour, and 24 hours.

### Create an alert rule

Fires when the `checkout` count exceeds 2 in the last 5 minutes:

```bash
curl -X POST http://localhost:3000/alert-rules \
  -H 'Content-Type: application/json' \
  -H 'x-api-key: <your_api_key>' \
  -d '{"event_type":"checkout","rule_condition":"above","threshold":2,"time_window":"5m"}'
```

`rule_condition` accepts `"above"` or `"below"`. `time_window` accepts `"5m"`, `"15m"`, `"1h"`, or `"24h"`.

### Check alert history

The background worker polls every 10 seconds. After the threshold is crossed:

```bash
curl http://localhost:3000/alerts -H 'x-api-key: <your_api_key>'
```

Returns alert events with `triggered_at` and `resolved_at` (null while open). Alerts auto-resolve when the condition clears.

### API reference

All endpoints except `/health` and `POST /tenants` require an `x-api-key` header.

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/tenants` | no | Create a tenant, get back a raw API key (shown once) |
| POST | `/events` | yes | Ingest an event |
| GET | `/events` | yes | List the last 100 events for this tenant |
| GET | `/metrics` | yes | Rolling window counts per event type (5m / 15m / 1h / 24h) |
| POST | `/alert-rules` | yes | Create a threshold alert rule |
| GET | `/alert-rules` | yes | List active alert rules |
| DELETE | `/alert-rules/{id}` | yes | Soft-delete an alert rule |
| GET | `/alerts` | yes | Alert event history (last 100) |
| GET | `/health` | no | Service health check |

### How it works

- **Ingestion** - `POST /events` writes to Postgres, then spawns a background task to update a Redis sorted set, so ingestion latency is not blocked by Redis.
- **Rolling windows** - Redis sorted sets keyed by `metrics:{tenant_id}:{event_type}`, scored by timestamp in ms. `ZCOUNT` over a window gives an exact rolling count; entries older than 24 hours are pruned on each write.
- **Alerting** - A Tokio background task polls all active rules every 10 seconds. It is edge-triggered: it fires once when the condition is first met and resolves automatically when it clears. Rules are soft-deleted to preserve history.
- **Multi-tenancy** - All reads and writes are scoped to the `tenant_id` resolved from the API key.

## Contributing

Contributions are welcome.

1. Fork the repo and create a branch from `main` (`git checkout -b feat/my-change`).
2. Make your change, keeping it focused. Match the style of the surrounding code.
3. Before opening a PR, make sure everything builds and lints cleanly:
   ```bash
   # backend/
   cargo fmt && cargo clippy && cargo test

   # frontend/
   npm run lint && npm run build
   ```
4. To add a database migration, run `sqlx migrate add <description>` from `backend/`.
5. Open a pull request against `main` describing what changed and why.

For bugs or feature ideas, open an issue first so the approach can be discussed.

## Project layout

```
backend/    Rust / Axum API, Postgres migrations, alert worker
frontend/   React / TypeScript (Vite) dashboard
```
