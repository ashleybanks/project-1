## ADDED Requirements

### Requirement: Site creation with unique token generation
An authenticated user SHALL be able to create a new site by providing a display name and the domain they intend to track. The system SHALL generate a cryptographically random, URL-safe token (minimum 16 bytes of entropy) as the site's identifier.

#### Scenario: Successful site creation
- **WHEN** an authenticated user submits a valid site name and domain
- **THEN** a site record is created in PostgreSQL with the generated token, and the user is shown the embed snippet

#### Scenario: Duplicate domain for same user
- **WHEN** a user attempts to create a second site with a domain they already track
- **THEN** the system returns a validation error

---

### Requirement: Embed snippet delivery
The system SHALL display the embed snippet for a site on demand. The snippet SHALL be a self-contained `<img>` tag with `display:none` targeting the tracking pixel endpoint with the site's token as the `s` parameter.

#### Scenario: Snippet displayed after creation
- **WHEN** a site is created successfully
- **THEN** the user is shown a copyable snippet: `<img src="https://t.<domain>/p?s=<token>&u=PAGE_URL" style="display:none" />`

#### Scenario: Snippet accessible from site settings
- **WHEN** a user navigates to their site's settings page
- **THEN** the embed snippet is displayed for copying

---

### Requirement: Workers KV synchronisation on site creation
When a new site is created, the system SHALL write the site token as a key to the Cloudflare Workers KV namespace used by the edge worker for token validation. This SHALL occur within the same request that creates the site in PostgreSQL, with a best-effort retry on KV write failure.

#### Scenario: KV write succeeds
- **WHEN** a site is created and the KV write succeeds
- **THEN** the edge worker can validate the new token within the KV propagation window (seconds)

#### Scenario: KV write fails
- **WHEN** the KV write fails after retries
- **THEN** the site is still created in PostgreSQL; a background job SHALL retry the KV write; the user is not shown an error

---

### Requirement: Workers KV synchronisation on site deletion
When a site is deleted, the system SHALL remove the site token from the Cloudflare Workers KV namespace so the edge worker stops accepting events for that token.

#### Scenario: Site deleted
- **WHEN** a user deletes a site
- **THEN** the KV key for that token is removed and subsequent beacon requests with that token are silently dropped at the edge

---

### Requirement: List sites for authenticated user
An authenticated user SHALL be able to retrieve a list of all sites they own, including display name, domain, token, creation date, and a summary of recent activity (total pageviews in the last 7 days).

#### Scenario: User with multiple sites
- **WHEN** an authenticated user requests their site list
- **THEN** all sites owned by that user are returned, ordered by creation date descending

#### Scenario: User with no sites
- **WHEN** an authenticated user has not created any sites
- **THEN** an empty list is returned and the UI prompts site creation

---

### Requirement: Site deletion
An authenticated user SHALL be able to delete a site they own. Deletion SHALL remove the site record from PostgreSQL and SHALL queue removal of associated analytics data from ClickHouse. Deletion is irreversible.

#### Scenario: Successful deletion
- **WHEN** a user confirms deletion of a site
- **THEN** the site record is removed from PostgreSQL, the KV token is invalidated, and a ClickHouse data purge is queued

#### Scenario: Deletion of another user's site
- **WHEN** a user attempts to delete a site they do not own
- **THEN** the system returns HTTP 403 and does not delete the site
