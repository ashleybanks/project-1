## Why

Existing web analytics tools either require cookie consent banners (making them friction-heavy for simple sites) or are expensive SaaS products with opaque data practices. There is an opportunity to build a lightweight, consent-free analytics service — privacy-by-design, not privacy-as-feature — targeting developers who host simple sites on GitHub Pages, Notion, and similar platforms.

## What Changes

This is a greenfield SaaS product. No existing codebase is being modified.

- New embed snippet: a single `<img>` tag users paste into their site to activate tracking
- New tracking pixel endpoint: a Cloudflare Edge Worker (Rust/WASM) that receives beacon requests, reads headers, validates site tokens, enqueues events, and responds with a 1×1 GIF
- New event processing pipeline: a Rust service that consumes queued events, anonymises all PII, enriches with geo/device data, and persists to ClickHouse
- New analytics query API: HTTP endpoints (Axum) exposing aggregated analytics to the dashboard
- New user accounts and site management: sign-up, login, site creation, snippet delivery, and billing scaffolding via Next.js + BetterAuth + PostgreSQL
- New analytics dashboard: client-rendered SPA at `app.*` for visualising pageviews, referrers, countries, devices, and top pages per site
- New marketing site: SSR pages at `www.*` covering landing page, pricing, and documentation

## Capabilities

### New Capabilities

- `tracking-pixel`: Cloudflare Edge Worker (Rust/WASM) serving the beacon endpoint — validates site token via Workers KV, reads request headers, enqueues raw event to Cloudflare Queues, responds with 1×1 GIF
- `event-processing`: Rust service consuming Cloudflare Queues — anonymises PII (daily-salted hash for unique visitors, IP used only for geo then discarded, UA parsed then discarded), writes enriched events to ClickHouse
- `analytics-api`: Axum HTTP API (same Rust service as event-processing) exposing query endpoints for the dashboard — pageviews over time, top pages, referrers, countries, devices, unique visitors
- `user-auth`: BetterAuth-powered sign-up, login, and session management backed by PostgreSQL; integrated into the Next.js app
- `site-management`: site CRUD (create/rename/delete), unique site token generation, embed snippet delivery, and KV cache synchronisation to the edge for token validation
- `analytics-dashboard`: Next.js `app.*` subdomain — client-rendered React SPA querying the analytics API; charts for pageviews, referrers, geography, device breakdown, and top pages
- `marketing-site`: Next.js `www.*` subdomain — SSR marketing pages covering landing, pricing, and documentation

### Modified Capabilities

## Impact

- New monorepo with three primary workspaces: `edge-worker` (Rust/WASM), `analytics-service` (Rust), `web` (Next.js)
- External dependencies: Cloudflare (Workers, Queues, KV), ClickHouse Cloud, PostgreSQL (Neon for dev), Fly.io or Railway for Rust service hosting, Vercel or Fly.io for Next.js
- No existing systems impacted — greenfield
