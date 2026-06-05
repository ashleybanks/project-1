use clickhouse::Row;
use serde::Serialize;

#[derive(Row, Serialize)]
pub struct PageviewRow {
    pub site_id: String,
    pub timestamp: i64,
    pub page_url: Option<String>,
    pub referrer_domain: Option<String>,
    pub country: String,
    pub browser_family: String,
    pub os_family: String,
    pub device_type: String,
    pub visitor_hash: String,
    pub asn: String
}

pub async fn insert_pageview(
    client: &clickhouse::Client,
    row: PageviewRow,
) -> anyhow::Result<()> {
    let mut insert = client.insert::<PageviewRow>("pageviews").await?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(())
}
