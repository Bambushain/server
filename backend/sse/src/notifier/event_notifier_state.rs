use std::sync::Arc;

use crate::notifier::event::EventBroadcaster;
use actix_web::{web, Responder};
use bamboo_common::core::entities::grove::Model;
use bamboo_common::core::entities::user::BambooUser;
use bamboo_common::core::queueing::EventAction;

#[derive(Clone)]
pub struct EventNotifierState {
    event_broadcaster: Arc<EventBroadcaster>,
}

impl EventNotifierState {
    pub(crate) async fn send_event(&self, event_action: EventAction, groves: Vec<Model>) {
        self.event_broadcaster
            .send_event(event_action, groves)
            .await
    }
}

impl EventNotifierState {
    pub fn new() -> Self {
        let event_broadcaster = EventBroadcaster::create();

        Self { event_broadcaster }
    }

    pub async fn new_client(&self, user: BambooUser) -> impl Responder + use < > {
        log::info!("Wanted new client");
        Arc::clone(&self.event_broadcaster).new_client(user).await
    }
}

impl Default for EventNotifierState {
    fn default() -> Self {
        Self::new()
    }
}

pub type EventNotifier = web::Data<EventNotifierState>;
