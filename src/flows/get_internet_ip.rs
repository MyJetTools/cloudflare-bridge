use flurl::FlUrl;

use crate::app::AppContext;

pub async fn get_internet_ip(app: &AppContext) -> String {
    let domain_zone = app.settings_reader.get_ip_url().await;

    let fl_url = FlUrl::new(domain_zone.as_str());

    let result = fl_url.get().await.unwrap();

    let result = result.receive_body().await.unwrap();

    let result = String::from_utf8(result).unwrap();

    result
}
