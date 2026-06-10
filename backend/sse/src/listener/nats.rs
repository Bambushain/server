use crate::notifier::{EventNotifierState, NotificationsNotifierState};
use async_nats::{Client, Message};
use bamboo_common::backend::database::get_database;
use bamboo_common::backend::dbal;
use bamboo_common::backend::mq::{get_nats, Queue};
use bamboo_common::core::queueing::{EventAction, FromMessage, Notification, NotificationError};
use futures_util::StreamExt;
use sea_orm::DatabaseConnection;
use tokio::task::JoinHandle;

pub async fn start_listening(
    event_notifier_state: EventNotifierState,
    notification_notifier_state: NotificationsNotifierState,
) -> Result<JoinHandle<()>, NotificationError> {
    log::info!("Start listening to events on nats");
    let db = get_database()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))?;
    let nats_client = get_nats().await?;

    Ok(tokio::spawn(spawn_listen(
        db.clone(),
        nats_client,
        event_notifier_state,
        notification_notifier_state,
    )))
}

pub(crate) async fn spawn_listen(
    db: DatabaseConnection,
    nats_client: Client,
    event_notifier_state: EventNotifierState,
    notification_notifier_state: NotificationsNotifierState,
) {
    let event_subscriber = nats_client
        .subscribe(Queue::Events)
        .await
        .map_err(|err| NotificationError::new(err.to_string()));
    let notifications_subscriber = nats_client
        .subscribe(Queue::Notifications)
        .await
        .map_err(|err| NotificationError::new(err.to_string()));

    if let Ok(mut event_subscriber) = event_subscriber
        && let Ok(mut notifications_subscriber) = notifications_subscriber
    {
        loop {
            tokio::select! {
                message = event_subscriber.next() => handle_event_message(message, &event_notifier_state, &db).await,
                message = notifications_subscriber.next() => handle_notification_message(message, &notification_notifier_state, &db).await,
                _ = tokio::signal::ctrl_c() => {
                    let _ = event_subscriber.unsubscribe().await;
                    let _ = notifications_subscriber.unsubscribe().await;
                    break;
                }
            }
        }
    }
}

async fn handle_event_message(
    message: Option<Message>,
    notifier_state: &EventNotifierState,
    db: &DatabaseConnection,
) {
    if let Some(message) = message
        && let Ok(event_action) =
            EventAction::from_message(message).map_err(|err| log::error!("{err}"))
        && let Ok(groves) = dbal::get_all_groves(db).await
    {
        notifier_state.send_event(event_action, groves).await;
    }
}

async fn handle_notification_message(
    message: Option<Message>,
    notifier_state: &NotificationsNotifierState,
    db: &DatabaseConnection,
) {
    if let Some(message) = message
        && let Ok(notification) =
            Notification::from_message(message).map_err(|err| log::error!("{err}"))
    {
        match notification {
            Notification::EventReminder(_, _)
            | Notification::GroveInviteEnable(_)
            | Notification::GroveInviteDisable(_) => {
                notifier_state.send_notification(notification, db).await
            }
            _ => {}
        }
    }
}
