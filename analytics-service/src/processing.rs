use blake3::Hasher;
use uaparser::{Client, Parser};

use crate::models::EventPayload;

pub static UA_REGEXES: &[u8] = include_bytes!("../regexes.yaml");

pub fn generate_hash(salt: [u8; 32], input: &EventPayload) -> String {
    let utc_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut hasher = Hasher::new_keyed(&salt);
    hasher.update(input.site_id.as_bytes());
    hasher.update(input.ip.as_bytes());
    hasher.update(input.user_agent.as_bytes());
    hasher.update(utc_date.as_bytes());
    hasher.finalize().to_string()
}

pub fn derive_device_type(client: &Client) -> &'static str {
    if client.device.family == "Spider" {
        return "bot";
    }
    match client.os.family.as_ref() {
        "iOS" | "Android" | "Windows Phone" | "BlackBerry OS" => {
            if client.device.family.contains("iPad")
                || client.device.family.contains("Tablet")
            {
                "tablet"
            } else {
                "mobile"
            }
        }
        _ => "desktop",
    }
}

pub fn normalise_referrer(referrer: Option<&str>) -> Option<String> {
    let r = referrer?;
    url::Url::parse(r).ok()?.host_str().map(|h| h.to_string())
}

pub fn parse_ua(parser: &uaparser::UserAgentParser, ua: &str) -> (String, String, &'static str) {
    let client = parser.parse(ua);
    let browser_family = client.user_agent.family.to_string();
    let os_family = client.os.family.to_string();
    let device_type = derive_device_type(&client);
    (browser_family, os_family, device_type)
}
