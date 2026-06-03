## ADDED Requirements

### Requirement: Site ownership authorisation
All analytics query endpoints SHALL verify that the authenticated user owns the requested `site_id`. Requests from unauthenticated users or users who do not own the site SHALL be rejected with HTTP 403.

#### Scenario: Owner queries their site
- **WHEN** an authenticated user requests analytics for a site they own
- **THEN** the API returns the requested data

#### Scenario: Non-owner attempts access
- **WHEN** an authenticated user requests analytics for a site they do not own
- **THEN** the API returns HTTP 403 with no data

#### Scenario: Unauthenticated request
- **WHEN** a request arrives without a valid session token
- **THEN** the API returns HTTP 401

---

### Requirement: Date range filtering
All analytics endpoints SHALL accept `from` and `to` query parameters (ISO 8601 date format, UTC). If omitted, the default range SHALL be the last 30 days.

#### Scenario: Custom date range provided
- **WHEN** the caller provides `from=2024-01-01&to=2024-01-31`
- **THEN** results are scoped to that date range inclusive

#### Scenario: Date range omitted
- **WHEN** no date parameters are provided
- **THEN** the API returns data for the last 30 days

---

### Requirement: Pageviews over time
The API SHALL expose `GET /api/sites/:site_id/pageviews` returning total pageviews and unique visitors aggregated by day within the requested date range.

#### Scenario: Successful pageviews query
- **WHEN** a valid request is made for a site with data
- **THEN** the response contains an array of `{ date, pageviews, unique_visitors }` objects

#### Scenario: Site with no data
- **WHEN** a valid request is made for a site with no events in the range
- **THEN** the response contains an empty array (not an error)

---

### Requirement: Top pages
The API SHALL expose `GET /api/sites/:site_id/pages` returning the top pages by pageview count within the date range, with a default limit of 10 and a maximum of 100.

#### Scenario: Successful top pages query
- **WHEN** a valid request is made
- **THEN** the response contains an array of `{ page_url, pageviews, unique_visitors }` ordered by pageviews descending

---

### Requirement: Top referrers
The API SHALL expose `GET /api/sites/:site_id/referrers` returning the top referrer domains by visit count within the date range.

#### Scenario: Successful referrers query
- **WHEN** a valid request is made
- **THEN** the response contains an array of `{ referrer_domain, visits }` ordered by visits descending; null referrers are grouped as "Direct / None"

---

### Requirement: Country breakdown
The API SHALL expose `GET /api/sites/:site_id/countries` returning pageview counts grouped by country code.

#### Scenario: Successful country query
- **WHEN** a valid request is made
- **THEN** the response contains an array of `{ country, pageviews }` ordered by pageviews descending

---

### Requirement: Device and browser breakdown
The API SHALL expose `GET /api/sites/:site_id/devices` returning counts grouped by device_type, browser_family, and os_family.

#### Scenario: Successful devices query
- **WHEN** a valid request is made
- **THEN** the response contains three arrays: `devices`, `browsers`, `operating_systems`, each with `{ name, pageviews }` ordered by pageviews descending

---

### Requirement: Summary stats
The API SHALL expose `GET /api/sites/:site_id/summary` returning aggregate totals for the date range: total pageviews, unique visitors, bounce rate (single-page sessions as a proportion of total sessions), and average time on site if derivable.

#### Scenario: Summary request
- **WHEN** a valid request is made
- **THEN** the response contains `{ pageviews, unique_visitors }` at minimum
