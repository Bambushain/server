use crate::mailer;
use bamboo_common::backend::services::EnvironmentService;
use bamboo_common::backend::{database, mailing};
use bamboo_common::core::queueing::NotificationError;

pub async fn start_listening() -> Result<(), NotificationError> {
    log::info!("Start mail handling");

    let db = database::get_database()
        .await
        .map_err(|err| NotificationError::new(format!("Failed to connect to database: {err}")))?;

    loop {
        tokio::select! {
            Ok(mails) = mailing::get_pending_mails(&db) => {
                for mail in mails {
                    log::info!("Received message: {}", &mail.id);
                    mailing::mark_sending(&mail, &db).await;
                    if let Err(err) = mailer::send_mail(&mail, EnvironmentService::new()).await {
                        log::error!("Failed to send email: {err}");
                        mailing::mark_failed(&mail, &err, &db).await
                    } else {
                        mailing::mark_sent(&mail, &db).await
                    }
                    tokio::select! {
                        _ = tokio::time::sleep(tokio::time::Duration::from_secs(3)) => {
                            continue;
                        }
                        _ = tokio::signal::ctrl_c() => {
                            break;
                        }
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}
