# CLAUDE.md

## Project

Privacy-friendly SaaS analytics service. Users embed a `<img>` tracking pixel on their sites; the system collects anonymised pageview data and displays it in a per-site analytics dashboard. No cookies, no raw PII stored — consent-free by design.

## Monorepo Structure

```
project-1/
├── edge-worker/          # Cloudflare Worker (Rust/WASM) — tracking pixel endpoint
├── analytics-service/    # Rust/Axum service — event processing + analytics query API
├── web/                  # Next.js app — marketing site (www.*) + dashboard (app.*)
├── infra/                # Docker Compose for local dev, DB migrations
│   ├── clickhouse/migrations/
│   └── postgres/migrations/
├── openspec/             # Planning artifacts (proposal, design, specs, tasks)
├── Cargo.toml            # Rust workspace root
└── Makefile              # Build and dev targets
```

## Build & Run

### Prerequisites
- Rust (stable) + `wasm-pack` + `worker-build` (`cargo install worker-build`)
- `wrangler` CLI (`npm install -g wrangler`)
- Docker + Docker Compose
- Node.js 20+

### Local dev infrastructure (ClickHouse + Postgres)
```bash
make dev-infra       # starts Docker Compose services
make stop-infra      # stops them
```

### Edge worker
```bash
make dev-edge        # wrangler dev (hot reload)
make build-edge      # production WASM build
```

### Analytics service
```bash
make dev-service     # cargo run (requires infra running)
make build-service   # release build
```

### Next.js web app
```bash
make dev-web         # next dev
cd web && npm run build  # production build
```

### Code quality
```bash
make check           # cargo check all workspaces
make fmt             # cargo fmt
make clippy          # cargo clippy -D warnings
```

## Architecture

```
Browser (visitor)
    │  GET /p?s=<token>&u=<page>
    ▼
Cloudflare Edge Worker (Rust/WASM)
    │  validate token via Workers KV
    │  enqueue event to Cloudflare Queues
    │  respond: 1×1 GIF
    ▼
Cloudflare Queues
    ▼
Analytics Service (Rust/Axum) — port 3001
    │  anonymise: IP+UA → daily hash, geo lookup, UA parse
    │  write to ClickHouse
    │  also serves: GET /api/sites/:id/* (dashboard query API)
    ▼
ClickHouse (port 8123)          PostgreSQL (port 5432)
analytics.pageviews table       users, sessions, sites tables

Next.js App (port 3000)
    www.*  → marketing pages (SSR)
    app.*  → dashboard SPA + auth + site management
```

## Key Design Decisions

See `openspec/changes/privacy-analytics-saas/design.md` for full rationale. Summary:
- **Rust/WASM** at the edge via `workers-rs` (learning goal + correct tool)
- **Cloudflare Queues** not Redis (native to Workers, no TCP from edge)
- **Single Rust service** for both event processing and query API (shared ClickHouse connection)
- **Daily salted HMAC hash** for unique visitor counting — no cookies, no stored PII
- **`<img>` tag** embed — works on Notion, GitHub Pages (no CSP issues)
- **Next.js** for web tier — BetterAuth is TypeScript-native, marketing site needs SSR

## Environment Variables

Each workspace has a `.env.example`. Copy and fill in:
- `edge-worker/.env.example` → `edge-worker/.dev.vars` (for `wrangler dev`)
- `analytics-service/.env.example` → `analytics-service/.env`
- `web/.env.example` → `web/.env.local`

## Specs

Full capability specs: `openspec/changes/privacy-analytics-saas/specs/`
Implementation tasks: `openspec/changes/privacy-analytics-saas/tasks.md`
