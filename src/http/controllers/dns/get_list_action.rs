use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use serde::Serialize;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/DnsZone",
    summary: "Get list of domains by dns zone",
    description: "Get list of domains by dns zone",
    controller: "DNS",
    input_data: GetListOfDomainsByDnsZoneHttpInput,

    result:[
        {status_code: 200, description: "Ok response"},
        {status_code: 401, description: "Domain is not setup"},
    ]
)]
pub struct GetListAction {
    app: Arc<AppContext>,
}

impl GetListAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetListAction,
    input_data: GetListOfDomainsByDnsZoneHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let domain_zone = action.app.get_domain_record(&input_data.domain).await?;

    let result = crate::cloud_flare_api::get_list(&domain_zone.zone_id, &domain_zone.api_key).await;

    if let Err(err) = &result {
        return Err(HttpFailResult::as_unauthorized(format!("{:?}", err).into()));
    }

    let result = result.unwrap();

    let result: Vec<CloudFlareDnsRecordHttpModel> = match result {
        Some(result) => {
            let mut response = Vec::with_capacity(result.len());

            for itm in result {
                response.push(CloudFlareDnsRecordHttpModel {
                    id: itm.id,
                    tp: itm.r#type,
                    name: itm.name,
                    content: itm.content,
                    proxied: itm.proxied,
                });
            }

            response
        }
        None => vec![],
    };

    HttpOutput::as_json(result).into_ok_result(false)
}

#[derive(MyHttpInput)]
pub struct GetListOfDomainsByDnsZoneHttpInput {
    #[http_query(description:"name of domain. Example: google.com")]
    pub domain: String,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct CloudFlareDnsRecordHttpModel {
    pub id: String,
    pub name: String,
    pub tp: String,
    pub content: String,
    pub proxied: bool,
}
