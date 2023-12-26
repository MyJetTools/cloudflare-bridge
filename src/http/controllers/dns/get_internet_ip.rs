use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/InternetIp",
    summary: "Get ip exposed to internet",
    description: "Get ip exposed to internet",
    controller: "DNS",

    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct GetInternetIpAction {
    app: Arc<AppContext>,
}

impl GetInternetIpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetInternetIpAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let ip = crate::flows::get_internet_ip(&action.app).await;
    HttpOutput::as_text(ip).into_ok_result(false)
}
