use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/DnsZone/ARecord",
    summary: "Create dns A-Record",
    description: "Create dns A-Record",
    controller: "DNS",
    input_data: CreateDomainRecordHttpInput,

    result:[
        {status_code: 200, description: "Ok response"},
        {status_code: 401, description: "Domain is not setup"},
    ]
)]
pub struct CreateAction {
    app: Arc<AppContext>,
}

impl CreateAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &CreateAction,
    input_data: CreateDomainRecordHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let domain_name = crate::utils::extract_domain_name(&input_data.domain);

    let domain_zone = action.app.get_domain_record(domain_name).await?;

    let ip = if let Some(ip) = input_data.ip {
        ip
    } else {
        crate::flows::get_internet_ip(&action.app).await
    };

    let result = crate::cloud_flare_api::create(
        &domain_zone.zone_id,
        &domain_zone.api_key,
        input_data.domain,
        input_data.proxied,
        ip,
    )
    .await;

    if let Err(err) = &result {
        return Err(HttpFailResult::as_unauthorized(format!("{:?}", err).into()));
    }

    HttpOutput::as_text(String::from_utf8(result.unwrap()).unwrap()).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct CreateDomainRecordHttpInput {
    #[http_body(description:"Full domain name")]
    pub domain: String,
    #[http_body(description:"Ip of service")]
    pub ip: Option<String>,
    #[http_body(description:"Cloudflare proxied")]
    pub proxied: bool,
}
