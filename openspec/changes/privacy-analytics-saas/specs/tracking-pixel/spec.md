## ADDED Requirements

### Requirement: Pixel endpoint responds with a valid 1×1 GIF
The worker SHALL respond to `GET /p` with a 1×1 transparent GIF image, Content-Type `image/gif`, and appropriate cache-control headers to prevent caching.

#### Scenario: Standard beacon request
- **WHEN** a browser requests `GET /p?s=<site_id>`
- **THEN** the worker responds with HTTP 200, Content-Type `image/gif`, and a 1×1 transparent GIF body

#### Scenario: Cache prevention
- **WHEN** the worker responds to any beacon request
- **THEN** the response includes `Cache-Control: no-store, no-cache` and `Pragma: no-cache`

---

### Requirement: Site token validation via Workers KV
The worker SHALL validate the `s` query parameter against a Cloudflare Workers KV namespace before enqueuing the event. Requests with absent or unrecognised tokens SHALL still receive the GIF response (to avoid exposing validation state to callers) but SHALL NOT enqueue an event.

#### Scenario: Valid site token
- **WHEN** the `s` parameter matches a key in the KV namespace
- **THEN** the worker enqueues the event and responds with the GIF

#### Scenario: Invalid or missing site token
- **WHEN** the `s` parameter is absent or not found in KV
- **THEN** the worker responds with the GIF but does not enqueue any event

---

### Requirement: Request header capture
The worker SHALL extract the following data from the incoming request for inclusion in the queued event: raw IP address, raw User-Agent string, Referer header (if present), request path of the page that loaded the pixel (passed as query parameter `u`), and site token.

#### Scenario: Full header set present
- **WHEN** the request includes IP, User-Agent, Referer, and `u` parameter
- **THEN** all fields are included in the queued event payload

#### Scenario: Optional fields absent
- **WHEN** Referer or `u` parameter is absent
- **THEN** the worker enqueues the event with those fields set to null/empty; it does not reject the request

---

### Requirement: Async event enqueue to Cloudflare Queues
The worker SHALL enqueue the captured event to a Cloudflare Queue. The GIF response SHALL NOT be delayed by the enqueue operation — the response MUST be sent concurrently with or before the enqueue completes.

#### Scenario: Enqueue succeeds
- **WHEN** a valid event is captured
- **THEN** a JSON payload containing site_id, ip, user_agent, referer, page_url, and timestamp is written to the queue

#### Scenario: Enqueue fails
- **WHEN** the Cloudflare Queue write fails
- **THEN** the worker still responds with the GIF; the error is logged but not surfaced to the caller

---

### Requirement: CORS headers for cross-origin requests
The worker SHALL include CORS headers permitting cross-origin `<img>` requests from any origin, as the pixel will be loaded from third-party sites.

#### Scenario: Cross-origin beacon request
- **WHEN** a request arrives with an `Origin` header from any domain
- **THEN** the response includes `Access-Control-Allow-Origin: *`
