## Context

Greenfield SaaS analytics product. No existing codebase. The system has three primary runtime components: a Cloudflare Edge Worker (hot path — serves every beacon request), a Rust analytics service (processes events and serves dashboard queries), and a Next.js web application (marketing, auth, site management, dashboard UI). These are developed as a monorepo with three workspaces.

The core constraint is privacy-by-design: no cookies, no raw PII stored, consent-free by legal basis. This shapes every data-handling decision.

## Goals / Non-Goals

**Goals:**
- Deliver a working end-to-end analytics pipeline: embed snippet → beacon → queue → processor → ClickHouse → dashboard
- Consent-free by design: no PII stored, no cookies, legally operable without consent banners
- Multi-tenant SaaS: user accounts, per-site token isolation, per-site dashboards
- Serve as a Rust/WASM learning project: meaningful Rust at both the edge and service layers

**Non-Goals:**
- Real-time (sub-second) analytics — near-real-time (seconds lag) is acceptable
- Custom event tracking beyond pageviews — script-based tracking is out of scope for v1
- Enterprise features (SSO, SAML, team accounts, audit logs)
- Self-hosted / open-core distribution model for v1
- Mobile SDK or native app tracking

## Decisions

### 1. Rust/WASM for the Edge Worker (vs TypeScript)

Cloudflare Workers natively run JS/TS. Rust reaches the Worker runtime via compilation to WASM using the `workers-rs` crate.

**Decision:** Rust/WASM via `workers-rs`.

**Rationale:** The project is explicitly a Rust learning exercise. The edge worker is a well-scoped Rust target — small surface area (validate token, read headers, enqueue, respond with GIF), interesting constraints (WASM target, no standard threads, async via JS Promises under the hood). The `workers-rs` ecosystem is mature and maintained by Cloudflare.

**Alternative considered:** TypeScript Worker — faster to write, but removes the learning goal and offers no meaningful performance advantage at this scale.

**Trade-off:** More complex build and deploy pipeline (wasm-pack, wrangler with WASM). Accepted.

---

### 2. Cloudflare Queues vs External Redis

**Decision:** Cloudflare Queues.

**Rationale:** Edge Workers cannot open arbitrary TCP connections to external hosts without a significant latency penalty (the edge PoP may be geographically distant from a Redis instance). Cloudflare Queues is purpose-built for Worker-to-Worker async messaging — the Worker enqueues without a network hop, and the queue handles durability and delivery. A standalone Rust service can consume from CF Queues via push delivery (HTTP webhook) or pull.

**Alternative considered:** Redis (Upstash or self-hosted) — familiar technology, but fundamentally mismatched with the edge runtime model.

---

### 3. Single Rust Service for Event Processing + Query API

**Decision:** The Rust analytics service handles both queue consumption (writes) and dashboard query API (reads) in a single Axum process.

**Rationale:** Both concerns share the same ClickHouse connection. Keeping them co-located avoids a second service, simplifies deployment, and makes ClickHouse the single source of truth accessed from one process. The service is I/O-bound (queue polling + CH queries), so Tokio handles both workloads cleanly.

**Alternative considered:** Separate processor and API services — unnecessary operational overhead for a side project at this stage.

---

### 4. ClickHouse for Analytics Data

**Decision:** ClickHouse Cloud (local instance for development).

**Rationale:** ClickHouse is purpose-built for the append-only, high-cardinality, aggregation-heavy query pattern of web analytics. Queries like "count pageviews by day for the last 90 days, grouped by country" execute in milliseconds against billions of rows. Logical multi-tenancy via `site_id` column is sufficient — no per-tenant table partitioning needed at this scale.

**Alternative considered:** TimescaleDB (PostgreSQL extension) — simpler ops, one fewer database technology, but meaningfully slower for analytics queries at scale and less suited to columnar compression.

---

### 5. PostgreSQL for Operational Data (Not ClickHouse)

**Decision:** PostgreSQL for users, sessions, sites, and plans.

**Rationale:** Operational data requires point lookups, updates, and transactions — patterns ClickHouse handles poorly (mutations are expensive; no true ACID transactions). A users table needs fast `WHERE email = ?` lookups; ClickHouse is not designed for this.

**Deployment:** Neon (serverless Postgres) for development and production. ClickHouse-managed Postgres is under evaluation for production (private preview at time of writing) — it offers native CDC sync from Postgres to ClickHouse via ClickPipes, which would be useful for syncing site metadata to the analytics layer.

---

### 6. Next.js for Web Tier (vs Full Rust Backend)

**Decision:** Next.js (App Router) for the web application, covering `www.*` (marketing) and `app.*` (dashboard) subdomains via middleware routing.

**Rationale:** The marketing site benefits from SSR and SEO. BetterAuth is TypeScript-native and the strongest self-hosted auth option for this stack. The dashboard pages are client-rendered React anyway (no SEO value from SSR). A single Next.js deployment handles both subdomains, shared auth session, and site management API routes. The interesting Rust work is already in the edge worker and analytics service.

**Alternative considered:** Axum API server + separate React SPA — removes BetterAuth as an option (requires WorkOS REST API), adds a second deployable unit, no meaningful benefit for a side project.

---

### 7. Daily Salted Hash for Unique Visitor Counting

**Decision:** On each event, compute `HMAC-SHA256(daily_salt, ip + user_agent + site_id + date)`. Store the hash. Discard the raw IP and user-agent immediately. Rotate the salt at midnight UTC daily.

**Rationale:** This enables unique visitor counting per day without storing any persistent identifier or PII. Because the salt rotates daily, yesterday's hash cannot be linked to today's — preventing cross-day tracking. This matches the Plausible Analytics approach and satisfies GDPR's "no personal data processed" basis, eliminating the need for consent banners.

---

### 8. `<img>` Tag Embed vs `<script>`

**Decision:** The embed snippet is a single `<img>` tag with `display:none`.

**Rationale:** Notion and GitHub Pages — two primary target hosting platforms — restrict external script execution via Content Security Policy. An `<img>` tag works universally. The downside (no SPA navigation tracking, single page-load event only) is acceptable for v1 targeting simple static sites.

---

### 9. Site Token Validation at the Edge via Workers KV

**Decision:** Validate incoming `site_id` tokens against Cloudflare Workers KV before enqueuing events.

**Rationale:** Prevents queue pollution from invalid or spoofed tokens. Workers KV is edge-local (low-latency reads), eventually consistent (a brief window where a newly-created site doesn't yet validate is acceptable). On site creation, the Next.js app writes the token to KV. On site deletion, it removes the token.

## Risks / Trade-offs

- **Rust/WASM build complexity** → Mitigated by `workers-rs` tooling and `wrangler`. Accept slower iteration on the edge worker.
- **Cloudflare Queues delivery guarantees** → CF Queues provides at-least-once delivery. The processor must be idempotent (duplicate events may be written to ClickHouse; acceptable for analytics, deduplicate in queries if needed).
- **KV eventual consistency** → A site token written by Next.js may take seconds to propagate to all edge PoPs. The first few beacon requests from a newly-created site may be dropped. Acceptable for v1.
- **Single analytics service (writes + reads)** → Under high write load, query latency may degrade. Mitigated by Tokio's async concurrency; can be split into separate services later if needed.
- **ClickHouse-managed Postgres in private preview** → Cannot rely on it for initial development. Neon is the fallback; architecture is designed so the switch requires only connection string changes.
- **Unique visitor accuracy** → The daily hash approach can over-count if a user switches networks or clears UA (rare) and under-count if multiple users share an IP (e.g., NAT). This is a known limitation of cookieless analytics, accepted and consistent with Plausible's approach.

## Open Questions

- **Domain name / branding** — not resolved; affects Cloudflare Worker routes and Next.js config
- **Pricing model** — free tier limits (sites? pageviews/month?) not yet defined; affects site management schema
- **ClickHouse-managed Postgres GA** — monitor for availability and pricing; may replace Neon in production
- **CF Queues consumer model** — push (HTTP webhook to Rust service) vs pull (Rust service polls); push is simpler but requires the Rust service to be publicly accessible
