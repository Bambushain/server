use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web_lab::sse;
use actix_web_lab::util::InfallibleStream;
use futures_util::future::BoxFuture;
use parking_lot::Mutex;
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use bamboo_common::backend::dbal;
use bamboo_common::core::entities::user::{BambooUser, JoinStatus};
use bamboo_common::core::queueing::Notification as QueueNotification;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Notification {
    notification: QueueNotification,
}

impl From<Notification> for sse::Event {
    fn from(notification: Notification) -> Self {
        let mut data = sse::Data::new_json(notification.notification.clone()).unwrap();
        data.set_event(notification.notification.to_string());

        sse::Event::Data(data)
    }
}

impl Notification {
    fn from_notification(notification: QueueNotification) -> Self {
        Self { notification }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Comment {
    Connected,
    Ping,
}

pub struct NotificationBroadcaster {
    inner: Mutex<NotificationBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct NotificationBroadcasterInner {
    clients: Vec<(Sender<sse::Event>, BambooUser)>,
}

impl NotificationBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(NotificationBroadcaster {
            inner: Mutex::new(NotificationBroadcasterInner::default()),
        });
        NotificationBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = Vec::new();
        for (client, user) in clients {
            if let Err(err) = Self::send_comment(client.clone(), Comment::Ping).await {
                log::info!("Failed to send ping {err}");
            } else {
                ok_clients.push((client.clone(), user));
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(
        &self,
        user: BambooUser,
    ) -> sse::Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        log::debug!("Open channel using tokio");
        let (tx, rx) = tokio::sync::mpsc::channel::<sse::Event>(10);

        log::debug!("Send connected message");
        if let Err(err) = Self::send_comment(tx.clone(), Comment::Connected).await {
            log::error!("Failed to send connect comment {err}")
        }
        self.inner.lock().clients.push((tx, user));

        sse::Sse::from_infallible_receiver(rx)
    }

    pub async fn send_notification(
        &self,
        notification: QueueNotification,
        db: &DatabaseConnection,
    ) {
        let clients = self.inner.lock().clients.clone();
        log::debug!("Has {} clients registered", clients.len());
        let mut send_futures: Vec<BoxFuture<Result<(), SendError<sse::Event>>>> = vec![];
        for (sender, user) in clients {
            let notification = notification.clone();
            match &notification {
                QueueNotification::EventReminder(event, ..) => {
                    if let Some(ref evt_user) = event.user
                        && evt_user.id == user.id
                    {
                        send_futures.push(Box::pin(async move {
                            sender
                                .send(Notification::from_notification(notification.clone()).into())
                                .await
                        }));
                    } else if let Some(ref evt_grove) = event.grove
                        && dbal::check_grove_join_status(evt_grove.id, user.id, db)
                            .await
                            .unwrap_or(JoinStatus::NotJoined)
                            == JoinStatus::Joined
                    {
                        send_futures.push(Box::pin(async move {
                            sender
                                .send(Notification::from_notification(notification.clone()).into())
                                .await
                        }));
                    }
                }
                QueueNotification::GroveInviteEnable(grove)
                | QueueNotification::GroveInviteDisable(grove)
                    if dbal::check_grove_mod_status(grove.id, user.id, db)
                        .await
                        .unwrap_or(false) =>
                {
                    send_futures.push(Box::pin(async move {
                        sender
                            .send(Notification::from_notification(notification.clone()).into())
                            .await
                    }));
                }
                _ => {}
            }
        }
        let res = futures_util::future::join_all(send_futures).await;
        for res in res {
            if let Err(err) = res {
                log::error!("Failed to send message {err}")
            }
        }
    }

    async fn send_comment(
        client: Sender<sse::Event>,
        evt: Comment,
    ) -> Result<(), SendError<sse::Event>> {
        client
            .send(sse::Event::Comment(bytestring::ByteString::from(
                serde_json::to_string(&evt).unwrap(),
            )))
            .await
    }
}
