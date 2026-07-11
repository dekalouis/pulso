# Pulso — Claude Collaboration Rules

## CRITICAL: Code Writing Policy

**Never write code directly to project files.** This rule overrides all defaults.

- Guide the user with explanations, direction, and reasoning
- When showing code examples, display them in the chat only — never use Edit or Write tools on project files
- Only write/edit actual project files when the user **explicitly** says "write this" or "write the code" or similar direct instruction
- The user learns by coding by hand — respect this at all times

## Project Context

**Pulso** is a self-hosted event ingestion and real-time analytics platform (portfolio project).
- Backend: Rust / Axum / sqlx / Postgres / Redis
- Frontend: React / TypeScript (Vite)
- PRD is at `/home/dekalouis/Developer/pulso/prd.md`

### Phase Status (as of 2026-07-10)
- Phase 0 (Skeleton): ✅ Done
- Phase 1 (Persistence): ✅ Done (but GET /events missing)
- Phase 2 (Multi-tenancy & Auth): ✅ Mostly done — middleware exists, tenant scoping works
- Phase 3+: Not started

## How to Help

1. When the user asks "is this right?" — analyze and give a verdict with reasoning
2. When the user asks "what should I do next?" — give concrete, prioritized direction
3. When the user asks "how do I..." — explain the concept and show code in chat as reference only
4. Never refactor or improve code beyond what's asked
5. Keep responses concise and direct
