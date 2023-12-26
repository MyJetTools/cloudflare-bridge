use flurl::{FlUrlError, IntoFlUrl};
use serde::*;

use super::{contracts::CreateModelResponse, CloudFlareError};

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFlareContract {
    pub result: Option<Vec<CloudFlareDnsRecord>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFlareDnsRecord {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    pub r#type: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: i32,
    pub locked: bool,
    pub meta: CloudFlareDnsRecordMeta,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFlareDnsRecordMeta {
    pub auto_added: bool,
    pub managed_by_apps: bool,
    pub managed_by_argo_tunnel: bool,
    pub source: String,
}

pub async fn get_list(
    domain_zone_id: &str,
    api_key: &str,
) -> Result<Option<Vec<CloudFlareDnsRecord>>, FlUrlError> {
    let mut response = "https://api.cloudflare.com"
        .append_path_segment("client")
        .append_path_segment("v4")
        .append_path_segment("zones")
        .append_path_segment(domain_zone_id)
        .append_path_segment("dns_records")
        .with_header("Authorization", format!("Bearer {api_key}"))
        .with_header("Content-Type", "application/json")
        .get()
        .await?;

    let result: CloudFlareContract = response.get_json().await?;

    Ok(result.result)
}

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
        .with_header("Authorization", format!("Bearer {api_key}"))
        .with_header("Content-Type", "application/json")
        .post_json(post_model)
        .await?;

    let result = response.receive_body().await?;

    let result_as_model: CreateModelResponse = serde_json::from_slice(&result).unwrap();

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
