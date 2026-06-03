## ADDED Requirements

### Requirement: Site selector and navigation
The dashboard SHALL allow a user with multiple sites to switch between them. The currently selected site SHALL be reflected in the URL (e.g., `/dashboard/<site_id>`).

#### Scenario: User with multiple sites
- **WHEN** a user navigates to the dashboard
- **THEN** they see a site selector and the first site is selected by default

#### Scenario: Direct navigation to site dashboard
- **WHEN** a user navigates directly to `/dashboard/<site_id>`
- **THEN** the dashboard loads for that site (if owned by the user)

---

### Requirement: Date range selector
The dashboard SHALL provide a date range selector with preset options (Today, Last 7 days, Last 30 days, Last 12 months) and a custom range picker. Changing the date range SHALL update all charts and tables without a full page reload.

#### Scenario: Preset range selected
- **WHEN** a user selects "Last 7 days"
- **THEN** all dashboard panels update to reflect data for that period

#### Scenario: Custom range selected
- **WHEN** a user selects specific from/to dates
- **THEN** all panels update for that custom range

---

### Requirement: Summary stats panel
The dashboard SHALL display at the top of the page: total pageviews, unique visitors, and top country for the selected period.

#### Scenario: Stats visible on load
- **WHEN** the dashboard loads for a site with data
- **THEN** the summary panel shows pageviews and unique visitors for the default date range

---

### Requirement: Pageviews over time chart
The dashboard SHALL display a time-series line or bar chart showing daily pageviews and unique visitors for the selected date range.

#### Scenario: Chart renders with data
- **WHEN** data exists for the selected range
- **THEN** the chart plots a data point for each day, with pageviews and unique visitors as separate series

#### Scenario: No data in range
- **WHEN** no events exist for the selected range
- **THEN** the chart displays an empty state message

---

### Requirement: Top pages table
The dashboard SHALL display a table of the top pages by pageview count for the selected period, showing page path, pageviews, and unique visitors. The table SHALL show at least 10 rows with an option to expand.

#### Scenario: Top pages populated
- **WHEN** data exists
- **THEN** pages are listed in descending order of pageviews

---

### Requirement: Top referrers table
The dashboard SHALL display a table of the top referrer domains. Direct / no-referrer traffic SHALL be labelled "Direct / None".

#### Scenario: Referrers populated
- **WHEN** referrer data exists
- **THEN** domains are listed in descending order of visits

---

### Requirement: Geography panel
The dashboard SHALL display a breakdown of pageviews by country, at minimum as a sorted list. A world map visualisation is desirable but not required for v1.

#### Scenario: Country data populated
- **WHEN** country data exists
- **THEN** countries are listed with their pageview counts in descending order

---

### Requirement: Devices, browsers, and OS breakdown
The dashboard SHALL display three breakdowns: device type (desktop/mobile/tablet), browser family, and operating system family, each as a simple ranked list with relative percentages.

#### Scenario: Device breakdown visible
- **WHEN** data exists
- **THEN** device types are shown with percentage of total pageviews

---

### Requirement: Authentication guard
All dashboard routes SHALL require an authenticated session. Unauthenticated requests SHALL be redirected to the login page.

#### Scenario: Unauthenticated dashboard access
- **WHEN** a non-authenticated user navigates to any `/dashboard` route
- **THEN** they are redirected to `/login`
