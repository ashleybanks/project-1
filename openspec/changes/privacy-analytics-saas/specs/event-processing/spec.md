## ADDED Requirements

### Requirement: Queue consumption
The analytics service SHALL consume events from the Cloudflare Queue via push delivery (HTTP POST to a configured endpoint) or pull polling, processing each event exactly once where possible (at-least-once delivery; idempotent processing is required).

#### Scenario: Event received from queue
- **WHEN** the service receives a queued event payload
- **THEN** it processes the event through the anonymisation and enrichment pipeline and writes the result to ClickHouse

#### Scenario: Duplicate event delivery
- **WHEN** the queue delivers the same event twice (at-least-once semantics)
- **THEN** both writes reach ClickHouse; downstream queries MUST handle deduplication or accept minor over-counting

---

### Requirement: PII anonymisation — unique visitor hash
The service SHALL compute a daily unique visitor identifier as `HMAC-SHA256(daily_salt, concat(ip, user_agent, site_id, utc_date))`. The daily salt SHALL rotate at midnight UTC. The raw IP and raw User-Agent SHALL be discarded immediately after this computation and SHALL NOT be persisted.

#### Scenario: Two requests from same visitor on same day
- **WHEN** two events arrive with identical IP, User-Agent, and site_id on the same UTC day
- **THEN** both produce the same visitor hash, enabling deduplication in queries

#### Scenario: Same visitor on different UTC days
- **WHEN** two events arrive from the same IP/UA/site on different UTC dates
- **THEN** the salt rotation produces different hashes, preventing cross-day linkability

#### Scenario: Raw PII not persisted
- **WHEN** an event is written to ClickHouse
- **THEN** the ClickHouse record contains the visitor hash but not the raw IP address or raw User-Agent string

---

### Requirement: Geo enrichment — IP to country
The service SHALL perform a geo IP lookup to map the raw IP address to a two-letter ISO 3166-1 country code. The IP SHALL be discarded after this lookup and SHALL NOT be written to storage.

#### Scenario: Known IP address
- **WHEN** the IP resolves to a known country
- **THEN** the ClickHouse record includes the two-letter country code

#### Scenario: Unknown or private IP address
- **WHEN** the IP cannot be mapped (private range, unknown)
- **THEN** the country field is written as null or an explicit "unknown" sentinel value

---

### Requirement: User-Agent enrichment — device and browser parsing
The service SHALL parse the raw User-Agent string to extract: browser family, OS family, and device type (desktop/mobile/tablet/bot). The raw User-Agent SHALL be discarded after parsing.

#### Scenario: Standard browser UA
- **WHEN** the User-Agent string identifies a known browser and OS
- **THEN** the ClickHouse record includes browser_family, os_family, and device_type

#### Scenario: Bot or crawler UA
- **WHEN** the User-Agent identifies a known bot or crawler
- **THEN** device_type is set to "bot"; the event SHALL still be written (filtering is a query-time concern)

---

### Requirement: ClickHouse event write
The service SHALL write one row per processed event to the ClickHouse `pageviews` table with the following columns: `site_id`, `timestamp`, `page_url`, `referrer_domain`, `country`, `browser_family`, `os_family`, `device_type`, `visitor_hash`.

#### Scenario: Successful write
- **WHEN** an event completes the processing pipeline
- **THEN** a single row is inserted into the `pageviews` table

#### Scenario: ClickHouse write failure
- **WHEN** the ClickHouse insert fails (network error, timeout)
- **THEN** the service retries with exponential backoff; after exhausting retries the event is logged to a dead-letter store and the queue message is not acknowledged (allowing redelivery)

---

### Requirement: Referrer normalisation
The service SHALL extract only the domain from the Referer header (stripping path, query string, and fragment) before storage.

#### Scenario: Full referrer URL present
- **WHEN** the event includes a Referer such as `https://news.ycombinator.com/item?id=12345`
- **THEN** only `news.ycombinator.com` is stored in `referrer_domain`

#### Scenario: Referrer absent
- **WHEN** the event has no Referer
- **THEN** `referrer_domain` is written as null
