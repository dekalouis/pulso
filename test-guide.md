# Pulso — Manual Test Guide

## Prerequisites

- Server running: `cargo run --bin backend` (from `backend/`)
- DB seeded: `cargo run --bin seed` (from `backend/`)

---

## 1. Health check (no auth required)

```bash
curl http://localhost:3000/health
```

Expected: `OK`

---

## 2. Ingest an event (valid API key)

```bash
curl -X POST http://localhost:3000/events \
  -H "Content-Type: application/json" \
  -H "x-api-key: test-key-123" \
  -d '{"event_type": "checkout"}'
```

Expected: `Event stored`

---

## 3. Reject missing API key (401)

```bash
curl -X POST http://localhost:3000/events \
  -H "Content-Type: application/json" \
  -d '{"event_type": "checkout"}'
```

Expected: `401 Unauthorized`

---

## 4. Reject wrong API key (401)

```bash
curl -X POST http://localhost:3000/events \
  -H "Content-Type: application/json" \
  -H "x-api-key: wrong-key" \
  -d '{"event_type": "checkout"}'
```

Expected: `401 Unauthorized`

---

## 5. Verify events are stored in DB

```bash
psql postgres://pulso:pulso@localhost:5433/pulso -c "SELECT * FROM events;"
```

Expected: rows with the correct `tenant_id` (`tenant-acme`) and `event_type`.

---

## Test API key

| Raw key       | Tenant       |
|---------------|--------------|
| test-key-123  | tenant-acme  |
