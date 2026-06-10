use std::sync::Arc;

use crate::notifier::notification::NotificationBroadcaster;
use actix_web::{web, Responder};
use bamboo_common::core::entities::user::BambooUser;
use bamboo_common::core::queueing::Notification;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct NotificationsNotifierState {
    notification_broadcaster: Arc<NotificationBroadcaster>,
}

impl NotificationsNotifierState {
    pub(crate) async fn send_notification(
        &self,
        notification: Notification,
        db: &DatabaseConnection,
    ) {
        self.notification_broadcaster
            .send_notification(notification, db)
            .await
    }
}

impl NotificationsNotifierState {
    pub fn new() -> Self {
        let notification_broadcaster = NotificationBroadcaster::create();

        Self {
            notification_broadcaster,
        }
    }

    pub async fn new_client(&self, user: BambooUser) -> impl Responder + use<> {
        log::info!("Wanted new client");
        Arc::clone(&self.notification_broadcaster)
            .new_client(user)
            .await
    }
}

impl Default for NotificationsNotifierState {
    fn default() -> Self {
        Self::new()
    }
}

pub type NotificationsNotifier = web::Data<NotificationsNotifierState>;
