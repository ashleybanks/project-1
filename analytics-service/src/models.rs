use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueuedMessage {
    pub account_id: String,
    pub queue: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub timestamp: String,
    pub body: EventPayload,
    pub attempts: u8,
}

#[derive(Debug, Deserialize)]
pub struct EventPayload {
    pub site_id: String,
    pub timestamp: String,
    pub ip: String,
    pub user_agent: String,
    pub country: String,
    pub asn: String,
    pub referrer: Option<String>,
    pub request_path: Option<String>,
}
