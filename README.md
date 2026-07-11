# Pulso

Pulso is a lightweight, self-hosted event ingestion and real-time analytics platform. Send events from your application to a single endpoint, scoped by tenant via API key, and query them back.

See [`prd.md`](./prd.md) for the full product spec and roadmap.

## Prerequisites

- Rust (via [rustup](https://rustup.rs)) — `cargo --version` should work
- PostgreSQL running locally (or in Docker) and reachable
- `sqlx-cli`, for running migrations:
  ```
  cargo install sqlx-cli --no-default-features --features postgres
  ```

## Database setup

1. Create a role and database for the project (adjust credentials as you like):
   ```
   psql -d postgres -c "CREATE USER pulso WITH PASSWORD 'pulso' SUPERUSER;"
   createdb -O pulso pulso
   ```
2. In `backend/`, create a `.env` file:
   ```
   DATABASE_URL=postgres://pulso:pulso@localhost:5432/pulso
   ```

## Running migrations

From `backend/`:
```
sqlx migrate run
```

Migrations also run automatically on server startup (`sqlx::migrate!` in `main.rs`), so this step is mainly useful for inspecting/creating migrations ahead of time.

To add a new migration:
```
sqlx migrate add <description>
```

## Running the backend

From `backend/`:
```
cargo run
```

Server listens on `http://localhost:3000`.

### Seeding a test API key

For local testing without going through tenant creation:
```
cargo run --bin seed
```
This inserts a fixed test key (`test-key-123`) for `tenant-acme`.

## Frontend

The frontend (`frontend/`) is a Vite + React + TypeScript scaffold — not yet wired to the backend. Once dependencies are installed:
```
cd frontend
npm install
npm run dev
```

## App flow so far

1. **Create a tenant** — `POST /tenants` with `{ "tenant_name": "..." }`. Returns a `tenant_id` and a raw `api_key`. The raw key is shown once; only its hash is stored.
2. **Ingest an event** — `POST /events` with header `x-api-key: <your key>` and body `{ "event_type": "..." }`. The `require_api_key` middleware resolves the key to a tenant and scopes the insert to it.
3. **List events** — `GET /events` with the same `x-api-key` header. Returns the last 100 events for that tenant only, most recent first.
4. **Health check** — `GET /health`, no auth required.

All tenant data is isolated: every query on `/events` is scoped by the `tenant_id` resolved from the API key, so one tenant can never read or write another tenant's events.

### Try it end to end

1. Create a tenant and grab the returned `api_key`:
   ```
   curl -X POST http://localhost:3000/tenants \
     -H 'Content-Type: application/json' \
     -d '{"tenant_name":"Acme Co"}'
   ```
   ```
   { "tenant_id": "tenant-...", "api_key": "...", "tenant_name": "Acme Co" }
   ```
2. Send an event using that key:
   ```
   curl -X POST http://localhost:3000/events \
     -H 'Content-Type: application/json' \
     -H 'x-api-key: <api_key from step 1>' \
     -d '{"event_type":"checkout"}'
   ```
3. List events for that tenant:
   ```
   curl http://localhost:3000/events -H 'x-api-key: <api_key from step 1>'
   ```

## Roadmap

Current status: Phase 0–2 (skeleton, persistence, multi-tenancy & auth) are functionally complete. Next up per the PRD: Phase 3 (Redis-backed rolling counters, `GET /metrics`). See [`prd.md`](./prd.md#11-roadmap--phases) for the full phase breakdown.
