use rust_extensions::ShortString;
use serde::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub get_ip_url: String,
    pub domain_zones: Vec<DomainZone>,
}

impl SettingsReader {
    pub async fn get_ip_url(&self) -> ShortString {
        let read_access = self.settings.read().await;
        ShortString::from_str(read_access.get_ip_url.as_str()).unwrap()
    }
    pub async fn get_by_domain(&self, domain: &str) -> Option<DomainZone> {
        let read_access = self.settings.read().await;

        for itm in &read_access.domain_zones {
            if rust_extensions::str_utils::compare_strings_case_insensitive(&itm.domain, domain) {
                return Some(itm.clone());
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainZone {
    pub domain: String,
    pub zone_id: String,
    pub api_key: String,
}
