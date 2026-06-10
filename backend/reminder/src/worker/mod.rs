use bamboo_common::backend::database::get_database;
use bamboo_common::backend::dbal;
use bamboo_common::backend::notification::notify;
use bamboo_common::core::entities::event::GroveEventReminder;
use bamboo_common::core::queueing::Notification;
use sea_orm::DatabaseConnection;

async fn handle_reminder(db: &DatabaseConnection) {
    let result = dbal::get_current_and_past_event_reminder(db).await;
    if let Err(err) = result {
        log::error!("Failed to get current and past event reminder: {err}");
    } else if let Ok(reminders) = result {
        log::debug!("Got {} outstanding reminders", reminders.len());
        for reminder in &reminders {
            if let Ok(event) = dbal::get_event_by_id(reminder.event_id, db).await {
                log::debug!("Notify reminder {} for event {}", reminder.id, reminder.event_id);
                let notification = Notification::EventReminder(
                    event,
                    GroveEventReminder {
                        id: reminder.id,
                        when: reminder.time,
                    },
                );
                notify(notification).await;
                if let Err(err) = dbal::mark_event_reminder_notified(reminder.event_id, db).await {
                    log::error!("Failed to mark notification as notified: {err}");
                }
            }
        }
    }
}

pub async fn start_work() -> std::io::Result<()> {
    log::info!("Start looking for event reminder");

    let db = get_database().await.map_err(std::io::Error::other)?;

    log::info!("Worker started");
    log::debug!("Make initial check for missed reminder");
    handle_reminder(&db).await;

    loop {
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_mins(1)) => {
                handle_reminder(&db).await
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}
