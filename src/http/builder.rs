use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(super::controllers::dns::GetListAction::new(
        app.clone(),
    )));

    result.register_get_action(Arc::new(super::controllers::dns::GetInternetIpAction::new(
        app.clone(),
    )));

    result.register_post_action(Arc::new(super::controllers::dns::CreateAction::new(
        app.clone(),
    )));

    result.register_delete_action(Arc::new(
        super::controllers::dns::DeleteDnsRecordAction::new(app.clone()),
    ));

    result
}
