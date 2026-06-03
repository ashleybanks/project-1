## 1. Monorepo Setup

- [ ] 1.1 Initialise git repository with monorepo structure: `edge-worker/`, `analytics-service/`, `web/`, `infra/`
- [ ] 1.2 Add root `Makefile` with targets for building and running all workspaces
- [ ] 1.3 Configure `.gitignore` for Rust (`target/`) and Next.js (`.next/`, `node_modules/`) artefacts
- [ ] 1.4 Update `CLAUDE.md` with project architecture, build commands, and workspace layout

## 2. Local Dev Infrastructure

- [ ] 2.1 Add Docker Compose file with ClickHouse and PostgreSQL services for local development
- [ ] 2.2 Write ClickHouse migration: create `pageviews` table (`site_id`, `timestamp`, `page_url`, `referrer_domain`, `country`, `browser_family`, `os_family`, `device_type`, `visitor_hash`)
- [ ] 2.3 Write PostgreSQL migration: create `users`, `sessions`, `sites` tables (Drizzle schema)
- [ ] 2.4 Configure Cloudflare account: create Workers KV namespace and Cloudflare Queue for local dev (via Wrangler)
- [ ] 2.5 Create `.env.example` files for each workspace documenting required environment variables

## 3. Edge Worker: Tracking Pixel

- [ ] 3.1 Initialise `edge-worker` Rust workspace with `workers-rs` crate and `wrangler.toml`
- [ ] 3.2 Implement `GET /p` handler: respond with 1×1 transparent GIF, `Cache-Control: no-store`, CORS headers
- [ ] 3.3 Implement Workers KV lookup to validate `s` (site token) query parameter; skip enqueue on invalid token
- [ ] 3.4 Implement request header extraction: IP (`CF-Connecting-IP`), `User-Agent`, `Referer`, `u` query param
- [ ] 3.5 Implement Cloudflare Queue enqueue with JSON event payload (site_id, ip, user_agent, referer, page_url, timestamp)
- [ ] 3.6 Ensure GIF response is not blocked by enqueue operation (fire-and-forget async)
- [ ] 3.7 Write unit tests for token validation logic and GIF response headers
- [ ] 3.8 Verify end-to-end with `wrangler dev` using a local test HTML page with the `<img>` snippet

## 4. Analytics Service: Event Processing

- [ ] 4.1 Initialise `analytics-service` Rust workspace with Tokio, Axum, and ClickHouse HTTP client
- [ ] 4.2 Implement CF Queues push consumer: Axum endpoint receiving queue delivery HTTP POST
- [ ] 4.3 Implement in-memory daily salt store with midnight UTC rotation using a Tokio background task
- [ ] 4.4 Implement unique visitor hash: `HMAC-SHA256(daily_salt, ip + user_agent + site_id + utc_date)`
- [ ] 4.5 Integrate MaxMind GeoLite2 (or equivalent) for IP-to-country lookup; confirm IP is not retained after lookup
- [ ] 4.6 Integrate User-Agent parser (e.g., `woothee` crate) for browser family, OS family, device type extraction
- [ ] 4.7 Implement referrer domain normalisation (extract host from URL, strip path/query/fragment)
- [ ] 4.8 Implement ClickHouse `pageviews` insert with exponential backoff retry on failure
- [ ] 4.9 Implement dead-letter logging for events that exhaust retries
- [ ] 4.10 Write unit tests for anonymisation pipeline: hash stability, salt rotation, PII non-persistence

## 5. Analytics Service: Query API

- [ ] 5.1 Implement session/JWT validation middleware on the Axum router
- [ ] 5.2 Implement site ownership check: verify requesting user owns the queried `site_id` (PostgreSQL lookup)
- [ ] 5.3 Implement date range query parameter parsing (`from`, `to`; default last 30 days)
- [ ] 5.4 Implement `GET /api/sites/:site_id/summary` (total pageviews, unique visitors)
- [ ] 5.5 Implement `GET /api/sites/:site_id/pageviews` (daily pageviews + unique visitors array)
- [ ] 5.6 Implement `GET /api/sites/:site_id/pages` (top pages by pageview count)
- [ ] 5.7 Implement `GET /api/sites/:site_id/referrers` (top referrer domains; null → "Direct / None")
- [ ] 5.8 Implement `GET /api/sites/:site_id/countries` (pageviews grouped by country code)
- [ ] 5.9 Implement `GET /api/sites/:site_id/devices` (device type, browser family, OS family breakdowns)
- [ ] 5.10 Write integration tests for all query endpoints against locally seeded ClickHouse data

## 6. Next.js App: Foundation

- [ ] 6.1 Initialise Next.js (App Router, TypeScript) in `web/` workspace
- [ ] 6.2 Install and configure BetterAuth with PostgreSQL adapter and Drizzle ORM
- [ ] 6.3 Implement `middleware.ts` for hostname-based routing: `www.*` → `/www/[...slug]`, `app.*` → `/app/[...slug]`
- [ ] 6.4 Set up Tailwind CSS and base UI component library (shadcn/ui)
- [ ] 6.5 Configure local environment variables and verify DB connection to local Postgres

## 7. Next.js App: User Auth

- [ ] 7.1 Build sign-up page (`/sign-up`) with email/password form and BetterAuth registration action
- [ ] 7.2 Build login page (`/login`) with email/password form and session creation
- [ ] 7.3 Implement email verification flow: send link on signup, verify on click, prompt banner for unverified users
- [ ] 7.4 Build forgot-password page and reset-password page (BetterAuth password reset flow)
- [ ] 7.5 Implement route guard in middleware: redirect unauthenticated users from `app.*` to `/login`
- [ ] 7.6 Build logout action (invalidates session) and add to navigation

## 8. Next.js App: Site Management

- [ ] 8.1 Build site creation form (display name, domain) with API route that generates token and writes to Postgres
- [ ] 8.2 Implement Cloudflare KV write helper called from site creation API route (on creation and deletion)
- [ ] 8.3 Build embed snippet display component: copyable `<img>` tag with the site's token
- [ ] 8.4 Build site list page (`/app/sites`) showing all user's sites with 7-day pageview summary
- [ ] 8.5 Build site settings page with rename option and delete action
- [ ] 8.6 Implement site deletion API route: remove from Postgres, remove from KV, queue ClickHouse data purge

## 9. Analytics Dashboard

- [ ] 9.1 Build dashboard layout: site selector, date range picker (presets + custom range)
- [ ] 9.2 Install charting library (Recharts) and set up base chart theme
- [ ] 9.3 Build summary stats panel (pageviews, unique visitors, top country)
- [ ] 9.4 Build pageviews-over-time chart (line/bar, daily granularity, dual series)
- [ ] 9.5 Build top pages table component
- [ ] 9.6 Build top referrers table component
- [ ] 9.7 Build country breakdown list component
- [ ] 9.8 Build devices / browsers / OS breakdown panels
- [ ] 9.9 Wire all components to analytics API using React Query; handle loading, error, and empty states
- [ ] 9.10 Verify dashboard renders correctly with seeded ClickHouse data

## 10. Marketing Site

- [ ] 10.1 Build landing page: headline, how-it-works section (snippet → analytics), privacy statement, sign-up CTA
- [ ] 10.2 Build pricing page (`/pricing`) with free tier and placeholder paid tier
- [ ] 10.3 Build docs quickstart page (`/docs/quickstart`) with copy-paste snippet instructions
- [ ] 10.4 Build docs privacy page (`/docs/privacy`) explaining anonymisation model and consent-free basis

## 11. Integration & End-to-End Verification

- [ ] 11.1 End-to-end smoke test: sign up → create site → copy snippet → load test page → confirm event appears in ClickHouse
- [ ] 11.2 Verify PII non-persistence: query ClickHouse and confirm no raw IP or User-Agent is stored
- [ ] 11.3 Test invalid site token: confirm edge worker drops event and returns GIF cleanly
- [ ] 11.4 Test daily salt rotation: seed events across midnight boundary and verify unique visitor hashes differ

## 12. Deployment

- [ ] 12.1 Provision ClickHouse Cloud instance and run `pageviews` table migration
- [ ] 12.2 Provision Neon (Postgres) production instance and run schema migrations
- [ ] 12.3 Write `Dockerfile` and `fly.toml` for `analytics-service`; deploy to Fly.io
- [ ] 12.4 Deploy Cloudflare Worker to production with production KV namespace and Queue bindings
- [ ] 12.5 Deploy Next.js app to Vercel with production environment variables
- [ ] 12.6 Configure DNS: `t.<domain>` → CF Worker, `www.<domain>` and `app.<domain>` → Next.js deployment
- [ ] 12.7 Run full end-to-end smoke test against production pipeline
