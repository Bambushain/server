use async_nats::Message;
use bamboo_common::backend::database::get_database;
use bamboo_common::backend::mq::{get_nats, Queue};
use bamboo_common::core::queueing::{FromMessage, Notification};
use bamboo_common::core::queueing::NotificationError;
use futures_util::StreamExt;
use sea_orm::DatabaseConnection;

pub async fn start_listening() -> std::io::Result<()> {
    log::info!("Start listening to events on nats");
    let db = get_database()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))
        .map_err(std::io::Error::other)?;
    let nats_client = get_nats().await.map_err(std::io::Error::other)?;

    let mut subscriber = nats_client
        .subscribe(Queue::Notifications)
        .await
        .map_err(std::io::Error::other)?;

    loop {
        tokio::select! {
            message = subscriber.next() => { handle_message(message, &db).await }
            _ = tokio::signal::ctrl_c() => {
                let _ = subscriber.unsubscribe().await;
                break;
            }
        }
    }

    Ok(())
}

async fn handle_message(message: Option<Message>, db: &DatabaseConnection) {
    if let Some(message) = message
        && let Ok(notification) =
            Notification::from_message(message).map_err(|err| log::error!("{err}"))
    {
        match notification {
            Notification::EventReminder(_, _)
            | Notification::GroveJoin(_, _)
            | Notification::GroveBan(_, _)
            | Notification::GroveUnban(_, _)
            | Notification::GroveModChange(_)
            | Notification::GroveDelete(_, _)
            | Notification::UserPasswordChange(_)
            | Notification::UserAccountDelete(_) => {
                bamboo_common::backend::mailing::enqueue_notification(&notification, db).await
            }
            Notification::GroveInviteEnable(_) => {
                log::debug!("Grove invites don't trigger emails");
            }
            Notification::GroveInviteDisable(_) => {
                log::debug!("Grove invites don't trigger emails");
            }
        }
    }
}
