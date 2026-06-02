use async_nats::Message;
use bamboo_common::backend::database::get_database;
use bamboo_common::backend::dbal;
use bamboo_common::backend::mq::{get_nats, Queue};
use bamboo_common::core::queueing::NotificationError;
use bamboo_common::core::queueing::{FromMessage, Notification};
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Error;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

pub async fn start_listening(fcm_service_account: String) -> std::io::Result<()> {
    log::info!("Start listening to events on nats");
    let db = get_database()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))
        .map_err(Error::other)?;
    let nats_client = get_nats().await.map_err(Error::other)?;

    let mut subscriber = nats_client
        .subscribe(Queue::Notifications)
        .await
        .map_err(Error::other)?;

    loop {
        tokio::select! {
            message = subscriber.next() => { handle_message(message, &fcm_service_account, &db).await }
            _ = tokio::signal::ctrl_c() => {
                let _ = subscriber.unsubscribe().await;
                break;
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct FcmMessage<N> {
    #[serde(rename = "type")]
    message_type: String,
    payload: N,
}

async fn handle_message(
    message: Option<Message>,
    fcm_service_account: &str,
    db: &DatabaseConnection,
) {
    if let Some(message) = message
        && let Ok(notification) =
            Notification::from_message(message).map_err(|err| log::error!("{err}"))
    {
        match notification {
            Notification::EventReminder(event, _) => {
                log::debug!("Event reminder notification received: {:?}", event);
                let payload = &FcmMessage {
                    message_type: "eventReminder".to_string(),
                    payload: event.clone(),
                };

                let tokens = if event.is_private
                    && let Some(ref user) = event.user
                    && let Ok(firebase_tokens) =
                        dbal::get_firebase_tokens_by_user(user.id, db).await
                {
                    firebase_tokens
                } else if !event.is_private
                    && let Some(ref grove) = event.grove
                    && let Ok(users) = dbal::get_all_users_by_grove(grove.id, db).await
                    && let Ok(tokens) = dbal::get_firebase_tokens_by_users(
                        users.into_iter().map(|user| user.id).collect(),
                        db,
                    )
                    .await
                {
                    tokens
                } else {
                    return;
                };

                if let Err(err) = send_firebase(
                    fcm_service_account,
                    tokens.into_iter().map(|token| token.token).collect(),
                    payload,
                    db,
                )
                .await
                {
                    log::error!("Failed to send firebase notification: {err}")
                }
            }
            Notification::GroveJoin(grove, panda) => {
                if let Ok(user_json) = serde_json::to_string(&panda)
                    && let Ok(grove_json) = serde_json::to_string(&grove)
                    && let Ok(tokens) = dbal::get_firebase_tokens_for_grove_mods(grove.id, db).await
                {
                    let mut data = HashMap::new();
                    data.insert("user", user_json);
                    data.insert("grove", grove_json);

                    let payload = &FcmMessage {
                        message_type: "groveJoin".to_string(),
                        payload: data,
                    };
                    if let Err(err) = send_firebase(
                        fcm_service_account,
                        tokens.into_iter().map(|token| token.token).collect(),
                        payload,
                        db,
                    )
                    .await
                    {
                        log::error!("Failed to send firebase notification: {err}")
                    }
                }
            }
            Notification::GroveInviteEnable(grove) => {
                if let Ok(grove_json) = serde_json::to_string(&grove)
                    && let Ok(tokens) = dbal::get_firebase_tokens_for_grove_mods(grove.id, db).await
                {
                    let mut data = HashMap::new();
                    data.insert("grove", grove_json);

                    let payload = &FcmMessage {
                        message_type: "groveInviteEnable".to_string(),
                        payload: data,
                    };
                    if let Err(err) = send_firebase(
                        fcm_service_account,
                        tokens.into_iter().map(|token| token.token).collect(),
                        payload,
                        db,
                    )
                    .await
                    {
                        log::error!("Failed to send firebase notification: {err}")
                    }
                }
            }
            Notification::GroveInviteDisable(grove) => {
                if let Ok(grove_json) = serde_json::to_string(&grove)
                    && let Ok(tokens) = dbal::get_firebase_tokens_for_grove_mods(grove.id, db).await
                {
                    let mut data = HashMap::new();
                    data.insert("grove", grove_json);

                    let payload = &FcmMessage {
                        message_type: "groveInviteDisable".to_string(),
                        payload: data,
                    };
                    if let Err(err) = send_firebase(
                        fcm_service_account,
                        tokens.into_iter().map(|token| token.token).collect(),
                        payload,
                        db,
                    )
                    .await
                    {
                        log::error!("Failed to send firebase notification: {err}")
                    }
                }
            }
            Notification::GroveBan(_, _)
            | Notification::GroveUnban(_, _)
            | Notification::GroveModChange(_)
            | Notification::GroveDelete(_, _)
            | Notification::UserPasswordChange(_)
            | Notification::UserAccountDelete(_) => {
                log::debug!("{notification} don't trigger fcm notifications")
            }
        }
    }
}

async fn send_firebase<T: Serialize>(
    fcm_service_account: &str,
    registration_ids: Vec<String>,
    payload: &FcmMessage<T>,
    db: &DatabaseConnection,
) -> Result<(), Error> {
    let mut data_map = HashMap::new();
    data_map.insert("payload".to_string(), serde_json::to_string(payload)?);

    let data_map = &data_map;

    let futures = registration_ids
        .into_iter()
        .map(|registration_id| async move {
            send_message(fcm_service_account, &registration_id, data_map, db).await
        });

    futures_util::future::join_all(futures).await;

    Ok(())
}

async fn send_message(
    fcm_service_account: &str,
    registration_id: &str,
    data: &HashMap<String, String>,
    db: &DatabaseConnection,
) -> Result<(), Error> {
    let secret = read_service_account_key(fcm_service_account)
        .await
        .map_err(Error::other)?;
    let project_id = secret
        .project_id
        .clone()
        .ok_or_else(|| Error::other("missing project_id"))?;
    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .map_err(Error::other)?;

    let token = auth
        .token(&["https://www.googleapis.com/auth/firebase.messaging"])
        .await
        .map_err(Error::other)?;
    let token_str = token.token().unwrap_or_default().to_string();
    let url = format!("https://fcm.googleapis.com/v1/projects/{project_id}/messages:send",);
    let client = Client::new();
    let body = serde_json::json!({
        "message": {
            "token": registration_id,
            "data": data
        }
    });

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {token_str}"))
        .json(&body)
        .send()
        .await;

    if let Err(err) = res {
        log::error!("Failed to send message: {err}");
        return Err(Error::other(err));
    } else if let Ok(res) = res {
        log::info!("Response status: {}", res.status());
        if !res.status().is_success() {
            return if res.status() == StatusCode::NOT_FOUND {
                log::info!("Clear the firebase token as it is probably invalid");
                let _ = dbal::delete_firebase_token(registration_id, db).await;
                Ok(())
            } else {
                let body = res.text().await;
                if let Ok(text) = body {
                    log::warn!("Response body: {}", text);
                }

                Ok(())
            };
        }
    }

    Ok(())
}
