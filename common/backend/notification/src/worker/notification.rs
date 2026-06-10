use bamboo_common_backend_mq::{publish, Queue};
use bamboo_common_core::queueing::Notification;

pub async fn notify(notification: Notification) {
    log::debug!("Push notification to nats");
    if let Err(err) = publish(Queue::Notifications, notification).await {
        log::error!("Failed to enqueue notification: {err}");
    } else {
        log::debug!("Enqueued notification");
    }
}
