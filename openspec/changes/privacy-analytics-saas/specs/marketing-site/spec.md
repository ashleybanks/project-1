## ADDED Requirements

### Requirement: Landing page
The marketing site SHALL have a landing page at `www.<domain>` communicating the product's value proposition: privacy-friendly, consent-free, lightweight analytics for simple sites. It SHALL include a primary call-to-action linking to sign-up.

#### Scenario: Visitor lands on homepage
- **WHEN** a visitor navigates to the root URL
- **THEN** they see the product headline, a brief description of how it works, and a visible sign-up CTA

---

### Requirement: Pricing page
The marketing site SHALL have a `/pricing` page describing available plans. For v1 this MAY describe a single free tier with placeholder detail for paid tiers.

#### Scenario: Pricing page accessible
- **WHEN** a visitor navigates to `/pricing`
- **THEN** they see the pricing structure and a CTA to sign up

---

### Requirement: Documentation
The marketing site SHALL include a `/docs` section with at minimum: a quickstart guide (how to add the snippet), a description of what data is collected and what is not, and a privacy FAQ explaining the consent-free model.

#### Scenario: Quickstart accessible
- **WHEN** a visitor navigates to `/docs/quickstart`
- **THEN** they see step-by-step instructions for adding the embed snippet to their site

#### Scenario: Privacy documentation accessible
- **WHEN** a visitor navigates to `/docs/privacy`
- **THEN** they see a clear description of the anonymisation approach, what is and is not stored

---

### Requirement: Subdomain routing
The Next.js application SHALL route `www.<domain>` requests to marketing pages and `app.<domain>` requests to the authenticated dashboard, using Next.js middleware hostname-based routing. Both subdomains SHALL be served from a single Next.js deployment.

#### Scenario: www subdomain serves marketing
- **WHEN** a request arrives for `www.<domain>`
- **THEN** marketing pages are served (landing, pricing, docs)

#### Scenario: app subdomain serves dashboard
- **WHEN** a request arrives for `app.<domain>`
- **THEN** dashboard routes are served with authentication enforcement
