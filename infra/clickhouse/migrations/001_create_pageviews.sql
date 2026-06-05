CREATE DATABASE IF NOT EXISTS analytics;

CREATE TABLE IF NOT EXISTS analytics.pageviews
(
    site_id          String,
    timestamp        DateTime64(3, 'UTC'),
    page_url         Nullable(String),
    referrer_domain  Nullable(String),
    country          String,
    browser_family   String,
    os_family        String,
    device_type      String,
    visitor_hash     String,
    asn   String
)
ENGINE = MergeTree()
ORDER BY (site_id, timestamp)
PARTITION BY toYYYYMM(timestamp)
SETTINGS index_granularity = 8192;
