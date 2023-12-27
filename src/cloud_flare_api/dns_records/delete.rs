use flurl::IntoFlUrl;

use crate::cloud_flare_api::{contracts::CloudFlareModelResponse, CloudFlareError};

pub async fn delete(
    domain_zone_id: &str,
    dns_record_id: &str,
    api_key: &str,
) -> Result<Vec<u8>, CloudFlareError> {
    let response = "https://api.cloudflare.com"
        .append_path_segment("client")
        .append_path_segment("v4")
        .append_path_segment("zones")
        .append_path_segment(domain_zone_id)
        .append_path_segment("dns_records")
        .append_path_segment(dns_record_id)
        .do_not_reuse_connection()
        .with_header("Authorization", format!("Bearer {api_key}"))
        .with_header("Content-Type", "application/json")
        .delete()
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
