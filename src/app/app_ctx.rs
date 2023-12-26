use std::sync::Arc;

use my_http_server::HttpFailResult;
use rust_extensions::AppStates;

use crate::settings_reader::DomainZone;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings_reader: Arc<crate::settings_reader::SettingsReader>,
}

impl AppContext {
    pub fn new(settings_reader: crate::settings_reader::SettingsReader) -> Self {
        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            settings_reader: Arc::new(settings_reader),
        }
    }

    pub async fn get_domain_record(&self, domain: &str) -> Result<DomainZone, HttpFailResult> {
        let domain_zone = self.settings_reader.get_by_domain(domain).await;

        if domain_zone.is_none() {
            return Err(HttpFailResult::as_unauthorized(
                "Domain is not setup".to_string().into(),
            ));
        }

        Ok(domain_zone.unwrap())
    }
}
