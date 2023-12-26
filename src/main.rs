use std::sync::Arc;

use app::AppContext;

mod app;
mod cloud_flare_api;
mod flows;
mod http;
mod settings_reader;
mod utils;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings_reader::SettingsReader::new(".cloudflare-bridge").await;
    let app = Arc::new(AppContext::new(settings_reader));
    crate::http::start(&app);
    app.app_states.wait_until_shutdown().await;
}
