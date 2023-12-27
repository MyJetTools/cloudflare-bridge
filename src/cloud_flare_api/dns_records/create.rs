use flurl::IntoFlUrl;
use serde::*;

use super::super::{contracts::CloudFlareModelResponse, CloudFlareError};

pub async fn create(
    domain_zone_id: &str,
    api_key: &str,
    name: String,
    proxied: bool,
    content: String,
) -> Result<Vec<u8>, CloudFlareError> {
    let post_model = CreateModel {
        content,
        name,
        proxied,
        r#type: "A".to_string(),
    };

    let response = "https://api.cloudflare.com"
        .append_path_segment("client")
        .append_path_segment("v4")
        .append_path_segment("zones")
        .append_path_segment(domain_zone_id)
        .append_path_segment("dns_records")
        .do_not_reuse_connection()
        .with_header("Authorization", format!("Bearer {api_key}"))
        .with_header("Content-Type", "application/json")
        .post_json(post_model)
        .await?;

    let result = response.receive_body().await?;

    let result_as_model: CloudFlareModelResponse = serde_json::from_slice(&result).unwrap();

    if let Some(mut errors) = result_as_model.errors {
        if errors.len() == 1 {
            return Err(errors.remove(0).into());
        }
    }

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateModel {
    pub content: String,
    pub name: String,
    pub proxied: bool,
    #[serde(rename = "type")]
    pub r#type: String,
}
