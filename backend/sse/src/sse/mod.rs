use actix_web::{get, Responder};

use crate::notifier::{EventNotifier, NotificationsNotifier};
use bamboo_common::backend::actix::middleware::{authenticate, Authentication};

#[get("/sse/event", wrap = "authenticate!()")]
pub async fn event_sse_client(
    notifier: EventNotifier,
    authentication: Authentication,
) -> impl Responder + use < > {
    log::debug!("Register new event sse client");
    notifier.new_client(authentication.user.clone()).await
}

#[get("/sse/notifications", wrap = "authenticate!()")]
pub async fn notifications_sse_client(
    notifier: NotificationsNotifier,
    authentication: Authentication,
) -> impl Responder + use < > {
    log::debug!("Register new notifications sse client");
    notifier.new_client(authentication.user.clone()).await
}
