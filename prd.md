# Pulse — Product Requirements Document

## 1. Overview

Pulse is a lightweight, self-hosted event ingestion and real-time analytics platform. It lets a business send arbitrary events (clicks, signups, API calls, checkouts, etc.) to a single endpoint, then view aggregated, real-time metrics and receive alerts when something looks off.

Conceptually, Pulse is a scoped-down version of tools like Mixpanel, PostHog, or Segment: multi-tenant, API-key authenticated, real-time, with a dashboard and threshold-based alerting.

## 2. Motivation

This is a portfolio project built to demonstrate backend systems skills that don't show up in typical CRUD-app portfolios: high-throughput ingestion, multi-tenancy, real-time aggregation, and observability. It's intentionally domain-agnostic (generic analytics infra, not fraud/rules-engine related) so it stands on its own as a general-purpose systems project.

Secondary goal: hands-on learning of Rust in a real, non-trivial backend context.

## 3. Goals

- Ingest events via a fast, simple HTTP API
- Support multiple isolated tenants (customers), each scoped to their own data via API key
- Aggregate event data in real time (counts, rates)
- Surface live metrics and trends in a dashboard
- Alert when a metric crosses a defined threshold
- Be genuinely deployed and demoable, not just runnable locally

## 4. Non-Goals

- Not a general-purpose data warehouse or long-term analytics store (no complex OLAP queries, no historical cohort analysis)
- Not built for billions of events; built to demonstrate architecture, not to scale to production SaaS volume
- No user-facing billing, plans, or payment integration
- No mobile SDKs; HTTP API only

## 5. Users / Use Cases

**Primary persona:** a small company wanting basic visibility into user behavior or system health without adopting a full commercial analytics suite.

**Example use case (demo):** a dummy e-commerce storefront sends `add_to_cart` and `checkout` events to Pulse. The store owner watches live checkout volume on the Pulse dashboard and gets alerted if checkout activity drops unexpectedly (signal of a broken flow).

## 6. Core Features

### 6.1 Event Ingestion
- `POST /events` accepts a JSON payload: `tenant_id` (derived from API key, not client-supplied), `event_type`, optional metadata payload
- Validates and persists the raw event
- Authenticated via per-tenant API key

### 6.2 Multi-Tenancy
- Each tenant has an isolated view of their own data
- API key maps to a tenant; all reads/writes are scoped by that tenant automatically
- No tenant can access another tenant's data under any circumstance

### 6.3 Real-Time Aggregation
- Rolling counters per tenant per event type (e.g., events in the last 1 min / 5 min / 1 hr)
- Aggregation updates in near real time, not batch-only

### 6.4 Alerting
- Threshold-based rules (e.g., "alert if `checkout` rate drops below X for Y minutes")
- Alert firing recorded and viewable in the dashboard

### 6.5 Dashboard
- Live chart of event volume over time
- Current alert status / alert history
- Basic tenant/API key management view

### 6.6 Observability (of Pulse itself)
- Tracing and metrics on the Pulse backend
- At least one real SLO-based alert on the service itself (e.g., ingestion latency, error rate)

## 7. Architecture

```
┌─────────────┐      POST /events       ┌──────────────────┐
│ Demo store  │ ───────────────────────▶ │  Pulse Backend    │
│ (event      │                          │  (Rust / Axum)    │
│  source)    │                          └─────────┬─────────┘
└─────────────┘                                    │
                                       ┌────────────┼─────────────┐
                                       ▼                          ▼
                                 ┌───────────┐            ┌──────────────┐
                                 │ Postgres  │            │ Redis        │
                                 │ (raw      │            │ (rolling     │
                                 │  events)  │            │  counters)   │
                                 └───────────┘            └──────────────┘
                                       ▲
                                       │ query
                                 ┌───────────┐
                                 │ React /TS │
                                 │ Dashboard │
                                 └───────────┘
```

## 8. Data Model (initial)

**events**
| column | type | notes |
|---|---|---|
| id | UUID | primary key |
| tenant_id | TEXT | derived from API key |
| event_type | TEXT | e.g. "checkout" |
| created_at | TIMESTAMPTZ | default now() |

**api_keys** *(Phase 2)*
| column | type | notes |
|---|---|---|
| key_hash | TEXT | hashed API key |
| tenant_id | TEXT | owner |
| created_at | TIMESTAMPTZ | |

**alerts** *(Phase 3)*
| column | type | notes |
|---|---|---|
| id | UUID | |
| tenant_id | TEXT | |
| rule_description | TEXT | |
| triggered_at | TIMESTAMPTZ | |

## 9. API Surface (initial)

- `POST /events` — ingest an event
- `GET /health` — service health check
- `GET /events?tenant_id=...` — list recent events (Phase 1)
- `GET /metrics?tenant_id=...` — aggregated metrics (Phase 3)
- `GET /alerts?tenant_id=...` — alert history (Phase 3)

## 10. Tech Stack

- **Backend:** Rust, Axum, Tokio, sqlx
- **Database:** PostgreSQL (raw event storage), Redis (real-time counters)
- **Frontend:** React, TypeScript
- **Auth:** per-tenant API key
- **Observability:** tracing crate, Prometheus/Grafana
- **Deployment:** Fly.io or Railway (backend), Vercel (frontend)

## 11. Roadmap / Phases

- **Phase 0 — Skeleton:** basic Axum server, `POST /events` (log only), `GET /health` ✅
- **Phase 1 — Persistence:** Postgres integration, real inserts, `GET /events` ✅
- **Phase 2 — Multi-tenancy & Auth:** API key table, middleware, tenant scoping on all queries
- **Phase 3 — Real-time aggregation:** Redis counters, rolling windows, `GET /metrics`
- **Phase 4 — Alerting:** threshold rules, alert storage, alert history endpoint
- **Phase 5 — Frontend dashboard:** live charts, alert view, tenant/API key management
- **Phase 6 — Demo e-commerce app:** standalone storefront emitting real events into Pulse
- **Phase 7 — Observability & deploy:** tracing, Grafana dashboard, SLO-based self-alerting, production deployment

## 12. Success Criteria

- End-to-end flow works: demo store → Pulse ingestion → real-time dashboard → alert firing
- Two tenants can use the system simultaneously with fully isolated data
- Deployed and publicly demoable (not just local)
- Clean, documented codebase suitable for a portfolio/interview walkthrough

## 13. Out of Scope (Future Considerations)

- Horizontal scaling / sharding for very high event volume
- Complex query language for custom analytics
- SDKs for mobile/other languages
- Billing/subscription tiers
