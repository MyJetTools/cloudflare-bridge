use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "DELETE",
    route: "/api/DnsZone/ARecord",
    summary: "Create dns A-Record",
    description: "Create dns A-Record",
    controller: "DNS",
    input_data: DeleteDomainRecordHttpInput,

    result:[
        {status_code: 200, description: "Ok response"},
        {status_code: 401, description: "Domain is not setup"},
    ]
)]
pub struct DeleteDnsRecordAction {
    app: Arc<AppContext>,
}

impl DeleteDnsRecordAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteDnsRecordAction,
    input_data: DeleteDomainRecordHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let domain_name = crate::utils::extract_domain_name(&input_data.domain);

    let domain_zone = action.app.get_domain_record(domain_name).await?;

    let result =
        crate::cloud_flare_api::delete(&domain_zone.zone_id, &input_data.id, &domain_zone.api_key)
            .await;

    if let Err(err) = &result {
        return Err(HttpFailResult::as_unauthorized(format!("{:?}", err).into()));
    }

    HttpOutput::as_text(String::from_utf8(result.unwrap()).unwrap()).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct DeleteDomainRecordHttpInput {
    #[http_query(description:"domain name")]
    pub domain: String,
    #[http_query(description:"id of record")]
    pub id: String,
}
